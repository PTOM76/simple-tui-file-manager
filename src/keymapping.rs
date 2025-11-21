use crossterm::event::{KeyCode, KeyEvent};

use crate::application::{App, Mode};

pub fn input_keybindings(code: KeyCode, mut line: String) -> String {
    match code {
        KeyCode::Enter => return line.to_owned(),
        KeyCode::Char(c) => line.push(c),
        KeyCode::Backspace => {
            line.pop();
        }
        KeyCode::Esc => {
            line.clear();
            return line;
        }
        _ => {}
    }
    line
}

pub fn main_keybindings(key: KeyEvent, mode: Mode, app: &mut App) -> bool {
    if mode != Mode::Normal {
        return false;
    }
    
    match (key.code) {
        KeyCode::Char('q') => return true,
        KeyCode::Char('j') | KeyCode::Down => {
            app.peek_selected_statefuldir().select_next();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.peek_selected_statefuldir().select_previous();
        }
        KeyCode::Char('h') | KeyCode::Left => app.move_to_parent_dir(),
        KeyCode::Char('l') | KeyCode::Right => app.move_to_child_dir(),
        KeyCode::Tab => app.next_dirtab(),
        KeyCode::BackTab => app.prev_dirtab(),
        _ => {}
    }

    false
}
