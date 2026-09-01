use std::collections::HashMap;
use crate::engine::{GameEngine, CombatInfo, GameUpdate, PlayerAction};
use crate::item::Owner;
use crate::house::{House, Room, RoomId, Direction};
use crate::map_state;
use crate::monster::MonsterId;
use ratatui::layout::{Layout, Constraint, Direction as LayoutDirection};

pub struct App {
    pub engine: GameEngine,
    pub mode: AppMode,
    pub message: String,
    pub should_quit: bool,
    pub inventory_state: ratatui::widgets::ListState,
}

pub struct CombatState {
    pub monster_id: MonsterId,
    pub monster_attacks_first: bool,
    pub state: CombatMenu,
}

pub enum CombatMenu {
    Main,
    ItemSelect,
}

pub enum AppMode {
    Exploring,
    Combat(CombatState),
}

impl App {
    pub fn new_game(engine: GameEngine, mode: AppMode) -> App {

        App{
            engine,
            mode,
            message: String::new(),
            should_quit: false,
            inventory_state: ratatui::widgets::ListState::default(),
        }
    }

    pub fn handle_key(app: &mut Self, key: crossterm::event::KeyEvent) {
        if let AppMode::Combat(_) = app.mode {
            Self::handle_combat_key(app, key);
            return;
        }
    
        match key.code {
            crossterm::event::KeyCode::Tab => {
                let count = app.game_state.registry.items_owned_by(Owner::Player).len();
                if count > 0 {
                    let i = match app.inventory_state.selected() {
                        Some(i) => (i + 1) % count,
                        None => 0
                    };
                    app.inventory_state.select(Some(i));
                }
                return;
            }

            crossterm::event::KeyCode::BackTab => {
                let count = app.game_state.registry.items_owned_by(Owner::Player).len();
                if count > 0 {
                    let i = match app.inventory_state.selected() {
                        Some(i) => (i + count - 1) % count,
                        None => 0,
                    };
                    app.inventory_state.select(Some(i));
                }
                return;
            }
            
            crossterm::event::KeyCode::Char('f') => {
                app.engine.apply_action(PlayerAction::EngageMonster);
                return;
            }

            crossterm::event::KeyCode::Char('s') => {
                app.engine.apply_action(PlayerAction::Search);
                return;
            }
            crossterm::event::KeyCode::Char('a') => {
                app.engine.apply_action(PlayerAction::PickUp);
                return;
            }
            crossterm::event::KeyCode::Char('d') => {
                match app.inventory_state.selected() {
                    Some(index) => {
                        let items = app.engine.game_state.registry.items_owned_by(Owner::Player);
                        match items.get(index) {
                            Some(item_id) => match app.game_state.drop_item(*item_id) {
                                Ok(()) => app.message = "Dropped it.".to_string(),
                                Err(_) => app.message = "Couldn't drop that.".to_string(),
                            },
                            None => app.message = "Nothing Selected".to_string(),
                        }
                    }
                    None => app.message = "Nothing selected".to_string(),
                };
                return;
            }
            
            crossterm::event::KeyCode::Char(' ') => {
                app.engine.apply_action(PlayerAction::EndTurn);
                return;
            }

            crossterm::event::KeyCode::Up => {
                app.engine.apply_action(PlayerAction::Move(Direction::North));
                return;
            }

            crossterm::event::KeyCode::Down => {
                app.engine.apply_action(PlayerAction::Move(Direction::South));
                return;
            }

            crossterm::event::KeyCode::Right => {
                app.engine.apply_action(PlayerAction::Move(Direction::East));
                return;
            }

            crossterm::event::KeyCode::Left => {
                app.engine.apply_action(PlayerAction::Move(Direction::West));
                return;
            }

            _ => {};
        }
    }
    pub fn handle_combat_key(app: &mut Self, key: crossterm::event::KeyEvent) {
        let (monster_id, monster_attacks_first) = match &app.mode {
            AppMode::Combat(state) => (state.monster_id, state.monster_attacks_first),
            AppMode::Exploring => return,
        };

        match key.code {
           crossterm::event::KeyCode::Char('f') => {
                let (outcome, log) = app.game_state.attack(monster_id, monster_attacks_first);
                app.message = log.join("\n");
                match outcome {
                    crate::combat::CombatOutcome::PlayerWon => {
                        app.mode = AppMode::Exploring;
                    }
                    crate::combat::CombatOutcome::PlayerDefeated => {
                        app.mode = AppMode::Exploring;
                        
                    }
                    crate::combat::CombatOutcome::Ongoing => {}
                    crate::combat::CombatOutcome::PlayerFled => {
                        app.mode = AppMode::Exploring;
                    }
                }
            }
            crossterm::event::KeyCode::Char('r') => {
                self.engine.apply_action(PlayerAction::Flee);
                app.mode = AppMode::Exploring;
            }
            _ => {}
        }
    }




    //data groupings to render
    pub fn render_player_stats(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &Self) {
        let chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([
                Constraint::Percentage(5),
                Constraint::Percentage(35),
                Constraint::Percentage(60),
            ])
            .split(area);
        let columns = Layout::default()
            .direction(LayoutDirection::Horizontal)
            .constraints([
                Constraint::Percentage(5),
                Constraint::Percentage(90),
                Constraint::Percentage(5),
            ])
            .split(chunks[2]);

        let hp_ratio = (app.game_state.player.health.max(0) as f64) / (app.game_state.player.max_health as f64);
        let hp_gauge = ratatui::widgets::Gauge::default()
            .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL))
            .gauge_style(ratatui::style::Style::default().fg(ratatui::style::Color::Red))
            .ratio(hp_ratio)
            .label(format!("\n  {}/{} HP", app.game_state.player.health, app.game_state.player.max_health));
        frame.render_widget(hp_gauge, chunks[1]);

        let player_stats = format!("\n  strength: {} \n  speed: {}\n  Moves Remaining: {}", app.game_state.player.strength, app.game_state.player.speed, app.game_state.player.moves_remaining);
        let player_paragraph = ratatui::widgets::Paragraph::new(player_stats);
        frame.render_widget(player_paragraph, columns[1]);

       let outer_block = ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .title(app.game_state.player.name.to_string());
       frame.render_widget(outer_block, area);
    }

    pub fn render_inventory(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &mut Self) {
        let item_ids = app.game_state.registry.items_owned_by(Owner::Player);
        let list_items: Vec<ratatui::widgets::ListItem> = item_ids.iter()
            .map(|id| {
                let name = app.game_state.registry.name_of(*id).unwrap_or("something");
                ratatui::widgets::ListItem::new(name.to_string())
            })
            .collect();
        
        let list = ratatui::widgets::List::new(list_items)
            .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).title("Inventory"))
            .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, area, &mut app.inventory_state);
    }

     fn render_log(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &mut Self) {
        let monster_names: Vec<String> = app.game_state.monsters.monsters_in(app.game_state.player.current_room)
                .iter().map(|m| m.name.clone()).collect();
        let monster_line = if monster_names.is_empty() {
            String::new()
        } else {
            format!("\nA {} growls, chained in the corner.", monster_names.join(", "))
        };
        let room = app.game_state.current_room().expect("current room should be valid");
        let exits: Vec<String> = room.exits.iter().map(|(d, _)| format!("{:?}", d)).collect();
        let app_log = format!("{}\n", app.message);
        let room_log = format!("Location: {}\nExits: {}\n{}{}\n", room.name, exits.join(", "), app_log, monster_line);
        let app_log_paragraph = ratatui::widgets::Paragraph::new(room_log)
            .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).title("Log:"));
        frame.render_widget(app_log_paragraph, area);
    }

       pub fn render_map(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &Self) {
        let positions = map_state::compute_positions(&app.game_state.house, RoomId(0));
        let (grid_w, grid_h) = map_state::grid_bounds(&positions);

        let cell_width = (area.width / grid_w as u16).max(3);
        let cell_height = (area.height / grid_h as u16).max(3);

        for (room_id, (x, y)) in &positions {
            let rect = ratatui::layout::Rect {
                x: area.x + (*x as u16) * cell_width,
                y: area.y + (*y as u16) * cell_height,
                width: cell_width,
                height: cell_height,
            };

            let highlighted_style = ratatui::style::Style::default().fg(ratatui::style::Color::Yellow);
            let normal_style = ratatui::style::Style::default().fg(ratatui::style::Color::White);
            
            let style = if *room_id == app.game_state.player.current_room {
                highlighted_style
            } else {
                normal_style
            };
            let room_names = app.game_state.house.room(*room_id).expect("current room should always be valid");
            let house_map = format!("{}", room_names.name);
            let house_map_paragraph = ratatui::widgets::Paragraph::new(house_map)
                .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL)
                .border_style(style));
            frame.render_widget(house_map_paragraph, rect);
        }
    } 
 
    fn render_combat_popup(frame: &mut ratatui::Frame, app: &App, state: &CombatState) {
        let popup_area = Self::centered_rect(70, 70 ,frame.area());
        frame.render_widget(ratatui::widgets::Clear, popup_area);

        let block = ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Red))
            .title("FIGHT!");
        frame.render_widget(&block, popup_area);
        let inner = block.inner(popup_area);

        let columns = Layout::default()
            .direction(LayoutDirection::Horizontal)
            .constraints([ Constraint::Percentage(30), Constraint::Percentage(40), Constraint::Percentage(40)])
            .split(inner);
        let monster_layout =  Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(80), Constraint::Percentage(10)])
            .split(columns[0]);


        let monster = app.game_state.monsters.monster(state.monster_id);
        let (m_name, m_health, m_max) = match monster {
            Ok(m) => (m.name.clone(), m.health.max(0), m.max_health),
            Err(_) => ("???".to_string(), 0, 1),
        };

        Self::render_player_stats(frame, columns[2], app);
        let monster_gauge =  ratatui::widgets::Gauge::default()
            .gauge_style(ratatui::style::Style::default().fg(ratatui::style::Color::Green))
            .ratio(m_health as f64 / m_max as f64)
            .label(format!("{}: {}/{} HP", m_name, m_health, m_max));
        frame.render_widget(monster_gauge, monster_layout[0]);

        let hint = ratatui::widgets::Paragraph::new("[f] Fight      [r] Flee")
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(hint, columns[1]);
    }

    //render groupings
    pub fn centered_rect(percent_x: u16, percent_y: u16, area: ratatui::layout::Rect) -> ratatui::layout::Rect {
        let popup_layout = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(area);

        Layout::default()
            .direction(LayoutDirection::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }

    pub fn render(frame: &mut ratatui::Frame, app: &mut App) {
        let columns = Layout::default()
            .direction(LayoutDirection::Horizontal)
            .constraints([
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ])
            .split(frame.area());
        let left = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ])
            .split(columns[0]);
        let right = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(columns[1]);
        
        Self::render_map(frame, left[0], app);
        Self::render_player_stats(frame, right[0], app);
        Self::render_inventory(frame, right[1], app);
        Self::render_log(frame, left[1], app);

        if let AppMode::Combat(state) = &app.mode {
            Self::render_combat_popup(frame, app, state);
        }
    }   
}
