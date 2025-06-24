use crate::app::{App, AreaEnum};
use crate::core::replace_placeholder;
use crossterm::event::{
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

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
                app.sql_input.clear();
                app.set_cursor_position(AreaEnum::Sql, 0);
            }
            KeyCode::Char(char) => {
                let pos = app.cursor_position(AreaEnum::Sql);
                app.sql_input.insert(pos, char);
                app.set_cursor_position(AreaEnum::Sql, pos + 1);
            }
            KeyCode::Backspace => {
                let pos = app.cursor_position(AreaEnum::Sql);
                if pos > 0 {
                    app.sql_input.remove(pos - 1);
                    app.set_cursor_position(AreaEnum::Sql, pos - 1);
                }
            }
            KeyCode::Delete => {
                let pos = app.cursor_position(AreaEnum::Sql);
                if pos < app.sql_input.len() {
                    app.sql_input.remove(pos);
                }
            }
            KeyCode::Left => {
                let pos = app.cursor_position(AreaEnum::Sql);
                if pos > 0 {
                    app.set_cursor_position(AreaEnum::Sql, pos - 1);
                }
            }
            KeyCode::Right => {
                let pos = app.cursor_position(AreaEnum::Sql);
                if pos < app.sql_input.len() {
                    app.set_cursor_position(AreaEnum::Sql, pos + 1);
                }
            }
            KeyCode::Home => {
                app.set_cursor_position(AreaEnum::Sql, 0);
            }
            KeyCode::End => {
                app.set_cursor_position(AreaEnum::Sql, app.sql_input.len());
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
                app.value_input.clear();
                app.set_cursor_position(AreaEnum::Value, 0);
            }
            KeyCode::Char(char) => {
                let pos = app.cursor_position(AreaEnum::Value);
                app.value_input.insert(pos, char);
                app.set_cursor_position(AreaEnum::Value, pos + 1);
            }
            KeyCode::Backspace => {
                let pos = app.cursor_position(AreaEnum::Value);
                if pos > 0 {
                    app.value_input.remove(pos - 1);
                    app.set_cursor_position(AreaEnum::Value, pos - 1);
                }
            }
            KeyCode::Delete => {
                let pos = app.cursor_position(AreaEnum::Value);
                if pos < app.value_input.len() {
                    app.value_input.remove(pos);
                }
            }
            KeyCode::Left => {
                let pos = app.cursor_position(AreaEnum::Value);
                if pos > 0 {
                    app.set_cursor_position(AreaEnum::Value, pos - 1);
                }
            }
            KeyCode::Right => {
                let pos = app.cursor_position(AreaEnum::Value);
                if pos < app.value_input.len() {
                    app.set_cursor_position(AreaEnum::Value, pos + 1);
                }
            }
            KeyCode::Home => {
                app.set_cursor_position(AreaEnum::Value, 0);
            }
            KeyCode::End => {
                app.set_cursor_position(AreaEnum::Value, app.value_input.len());
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
        AreaEnum::Sql => {
            let pos = app.cursor_position(AreaEnum::Sql);
            app.sql_input.insert_str(pos, &data);
            app.set_cursor_position(AreaEnum::Sql, pos + data.len());
        }
        AreaEnum::Value => {
            let pos = app.cursor_position(AreaEnum::Value);
            app.value_input.insert_str(pos, &data);
            app.set_cursor_position(AreaEnum::Value, pos + data.len());
        }
        _ => return,
    }
}

pub fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
        if let Some(clicked_area) = app.get_area_by_coordinate(mouse.column, mouse.row) {
            app.current_area = clicked_area;
            if clicked_area == AreaEnum::Result {
                app.result = replace_placeholder(app.sql_input.as_str(), app.value_input.as_str());
            }
        }
    }
}
