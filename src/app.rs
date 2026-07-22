use crate::game_state::GameState;
use crate::house::Direction;

pub struct App {
    pub game_state: GameState,
    pub message: String,
    pub should_quit: bool,
}

impl App {
    pub fn new_game(game_state: GameState) -> App {
        println!("Welcome {}! Please, make yourself at home", game_state.player.name);
        App{
            game_state,
            message: String::new(),
            should_quit: false,
        }
    }

    pub fn handle_key(app: &mut App, key: crossterm::event::KeyEvent) {
        let dir = match key.code {
            crossterm::event::KeyCode::Char('n') => Some(Direction::North),
            crossterm::event::KeyCode::Char('s') => Some(Direction::South),
            crossterm::event::KeyCode::Char('e') => Some(Direction::East),
            crossterm::event::KeyCode::Char('w') => Some(Direction::West),
            crossterm::event::KeyCode::Char('q') => { app.should_quit = true; return;}
            _ => None,
        };
        match dir {
            None => {app.message = "Oops! That's not an available direction. Please use n, s, e, w, or q.".to_string()}
            Some(d) => match app.game_state.move_player(d) {
                Ok(()) => app.message.clear(),
                Err(_) => app.message = "I think that's a wall... Maybe try another direction?".to_string(),
            }
        }
    }

    pub fn render(frame: &mut ratatui::Frame, app: &App) {
        let room = app.game_state.current_room().expect("current room should be valid");
        let exits: Vec<String> = room.exits.iter().map(|(d, _)| format!("{:?}", d)).collect();

        let text = format!("Location: {}\nExits: {}\n\n{}", room.name, exits.join(", "), app.message);
        let paragraph = ratatui::widgets::Paragraph::new(text)
            .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).title("Betrayal"));
        frame.render_widget(paragraph, frame.area());
    }
}
