use crate::game_state::GameState;
use crate::item::Owner;
use crate::house::Direction;
use crate::map_state::room_positions;
use crate::monster::MonsterId;
use ratatui::layout::{Layout, Constraint, Direction as LayoutDirection};

pub struct App {
    pub game_state: GameState,
    pub mode: AppState,
    pub message: String,
    pub should_quit: bool,
    pub inventory_state: ratatui::widgets::ListState,
}

pub struct CombatState {
    pub monster_id: MonsterId,
    pub monster_attacks_first: bool,
    pub menu: CombatMenu,
}

pub enum CombatMenu {
    Main,
    ItemSelect,
}

pub enum AppState {
    Exploring,
    Combat(CombatState),
}

impl App {
    pub fn new_game(game_state: GameState, mode: AppState) -> App {

        App{
            game_state,
            mode,
            message: String::new(),
            should_quit: false,
            inventory_state: ratatui::widgets::ListState::default(),
        }
    }

    pub fn handle_key(app: &mut Self, key: crossterm::event::KeyEvent) {
        match &app.mode{
            AppState::Combat(_) => {
                Self::handle_combat_key(app, key);
                return;
            }
            AppState::Exploring => {}
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

            crossterm::event::KeyCode::Char('s') => {
                match app.game_state.player.search_room(&app.game_state.registry) {
                    Ok(()) => {
                        let item_id = app.game_state.player.found_item
                            .expect("search_room set found_item on Ok");
                        let name = app.game_state.registry.name_of(item_id).unwrap_or("something");
                        app.message = format!("You found: {}", name);
                    }
                    Err(_) => app.message = "There's nothing here.".to_string(),
                };
                return;
            }
            crossterm::event::KeyCode::Char('a') => {
                match app.game_state.player.found_item {
                    Some(item_id) => match app.game_state.pick_up_item(item_id) {
                        Ok(()) => app.message = "Picked it up.".to_string(),
                        Err(_) => app.message = "Couldn't pick that up.".to_string(),
                    },
                    None => app.message = "Nothing to pick up — search first with '?'.".to_string(),
                };
                return;
            }
            crossterm::event::KeyCode::Char('d') => {
                match app.inventory_state.selected() {
                    Some(index) => {
                        let items = app.game_state.registry.items_owned_by(Owner::Player);
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
                app.game_state.end_turn();
                app.message = "you end your turn".to_string();
                return;
            }
            _ => {}
        }

        let dir =  match key.code {
            crossterm::event::KeyCode::Up => Some(Direction::North),
            crossterm::event::KeyCode::Down => Some(Direction::South),
            crossterm::event::KeyCode::Right => Some(Direction::East),
            crossterm::event::KeyCode::Left => Some(Direction::West),
            crossterm::event::KeyCode::Char('q') => { app.should_quit = true; return; }
            _ => None,
        };
        match dir {
            None => { app.message = "Oops! That's not an available direction. \nPlease use arrow keys or q.".to_string() },
            Some(d) => match app.game_state.move_player(d) {
                Ok(()) => {
                    app.message.clear();
                    let notices = app.game_state.process_events();
                    if !notices.is_empty() {
                        app.message = notices.join("\n");
                    }
                }
                Err(_) => app.message = "I think that's a wall...\nMaybe try another direction?".to_string(),
            },
        }
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

    pub fn render_map(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &Self) {
        let positions = room_positions();
        let cell_width = 12;
        let cell_height = 6;

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

    pub fn render(frame: &mut ratatui::Frame, app: &mut App) {
        let cols = Layout::default()
            .direction(LayoutDirection::Horizontal)
            .constraints([
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ])
            .split(frame.area());
        let left_col = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(cols[1]);
        let right_col = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(cols[0]);
        
        Self::render_map(frame, right_col[1], app);
        Self::render_inventory(frame, left_col[0], app);
        let monster_names: Vec<String> = app.game_state.monsters.monsters_in(app.game_state.player.current_room)
                .iter().map(|m| m.name.clone()).collect();
        let monster_line = if monster_names.is_empty() {
                String::new()
            } else {
                format!("\nA {} growls, chained in the corner.", monster_names.join(", "))
            };
        let room = app.game_state.current_room().expect("current room should be valid");
        let exits: Vec<String> = room.exits.iter().map(|(d, _)| format!("{:?}", d)).collect();
        let room_map = format!("Location: {}\nExits: {}{}\n", room.name, exits.join(", "), monster_line);
        let app_log = format!("{}\n", app.message);
        let rm_map_paragraph = ratatui::widgets::Paragraph::new(room_map)
            .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).title("Terminal Betrayal"));
        frame.render_widget(rm_map_paragraph, right_col[0]);
        let app_log_paragraph = ratatui::widgets::Paragraph::new(app_log)
            .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).title("Log:"));
        frame.render_widget(app_log_paragraph, left_col[1]);
    }
}
