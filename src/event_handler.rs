use crate::app::{App, AreaEnum};
use crate::core::replace_placeholder;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    match app.current_area {
        AreaEnum::Sql => match key.code {
            KeyCode::Tab => app.current_area = app.next_area(),
            KeyCode::BackTab => {
                app.current_area = app.prev_area();
                app.result = replace_placeholder(app.sql_input.as_str(), app.value_input.as_str());
            }
            KeyCode::Char('q') => app.should_exit = true,
            KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.sql_input.clear()
            }
            KeyCode::Char(char) => app.sql_input.push(char),
            KeyCode::Backspace => {
                app.sql_input.pop();
            }
            _ => return,
        },
        AreaEnum::Value => match key.code {
            KeyCode::Tab => {
                app.current_area = app.next_area();
                app.result = replace_placeholder(app.sql_input.as_str(), app.value_input.as_str());
            }
            KeyCode::BackTab => app.current_area = app.prev_area(),
            KeyCode::Char('q') => app.should_exit = true,
            KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.value_input.clear()
            }
            KeyCode::Char(char) => app.value_input.push(char),
            KeyCode::Backspace => {
                app.value_input.pop();
            }
            _ => return,
        },
        AreaEnum::Result => match key.code {
            KeyCode::Tab => app.current_area = app.next_area(),
            KeyCode::BackTab => app.current_area = app.prev_area(),
            KeyCode::Char('q') => app.should_exit = true,
            _ => return,
        },
    }
}

pub fn handle_paste(app: &mut App, data: String) {
    match app.current_area {
        AreaEnum::Sql => app.sql_input.push_str(data.as_str()),
        AreaEnum::Value => app.value_input.push_str(data.as_str()),
        _ => return,
    }
}
