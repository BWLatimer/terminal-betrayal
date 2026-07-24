use crate::game_state::GameState;
use crate::house::Direction;
use crate::map_state::room_positions;
use ratatui::layout::{Layout, Constraint, Direction as LayoutDirection};

pub struct App {
    pub game_state: GameState,
    pub message: String,
    pub should_quit: bool,
}

impl App {
    pub fn new_game(game_state: GameState) -> App {

        App{
            game_state,
            message: String::new(),
            should_quit: false,
        }
    }

    pub fn handle_key(app: &mut Self, key: crossterm::event::KeyEvent) {
            let dir = match key.code {
            crossterm::event::KeyCode::Char('n') => Some(Direction::North),
            crossterm::event::KeyCode::Char('s') => Some(Direction::South),
            crossterm::event::KeyCode::Char('e') => Some(Direction::East),
            crossterm::event::KeyCode::Char('w') => Some(Direction::West),
            crossterm::event::KeyCode::Char('q') => { app.should_quit = true; return;}
            _ => None,
        };
        match dir {
            None => {app.message = "Oops! That's not an available direction. \nPlease use n, s, e, w, or q.".to_string()}
            Some(d) => match app.game_state.move_player(d) {
                Ok(()) => app.message.clear(),
                Err(_) => app.message = "I think that's a wall...\nMaybe try another direction?".to_string(),
            }
        }
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
            let room_names = app.game_state.house.room(*room_id).expect("current room shouls always be valid");
            let house_map = format!("{}", room_names.name);
            let house_map_paragraph = ratatui::widgets::Paragraph::new(house_map)
                .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL)
                .border_style(style));
            frame.render_widget(house_map_paragraph, rect);
        }
    }

    pub fn render(frame: &mut ratatui::Frame, app: &App) {
        let chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ])
            .split(frame.area());
        
        Self::render_map(frame, chunks[1], app);
        let room = app.game_state.current_room().expect("current room should be valid");
        let exits: Vec<String> = room.exits.iter().map(|(d, _)| format!("{:?}", d)).collect();
        let room_map = format!("Location: {}\nExits: {}\n", room.name, exits.join(", "));
        let app_log = format!("{}\n", app.message);
        let rm_map_paragraph = ratatui::widgets::Paragraph::new(room_map)
            .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).title("Terminal Betrayal"));
        frame.render_widget(rm_map_paragraph, chunks[0]);
        let app_log_paragraph = ratatui::widgets::Paragraph::new(app_log)
            .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).title("Log:"));
        frame.render_widget(app_log_paragraph, chunks[2]);
    }
}
