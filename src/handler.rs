use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Selection movement
        KeyCode::Up => {
            app.move_up();
        }
        KeyCode::Down => {
            app.move_down();
        }
        KeyCode::Left => {
            app.move_left();
        }
        KeyCode::Right => {
            app.move_right();
        }
        // Grid operations
        KeyCode::Backspace | KeyCode::BackTab => {
            app.clear_cell();
        }
        KeyCode::Char('r') => {
            app.reset_grid();
        }
        KeyCode::Char(ch) => {
            if let Some(num) = ch.to_digit(10) {
                app.set_cell(num);
            }
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
