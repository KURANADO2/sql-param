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
            _other => handle_common_key(app, key),
        },
        AreaEnum::Value => match key.code {
            KeyCode::Tab => {
                app.current_area = app.next_area();
                app.result = replace_placeholder(app.sql_input.as_str(), app.value_input.as_str());
            }
            KeyCode::BackTab => app.current_area = app.prev_area(),
            _other => handle_common_key(app, key),
        },
        AreaEnum::Result => match key.code {
            KeyCode::Tab => app.current_area = app.next_area(),
            KeyCode::BackTab => app.current_area = app.prev_area(),
            KeyCode::Char('q') => app.should_exit = true,
            _ => return,
        },
    }
}

fn handle_common_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.should_exit = true,
        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input_clear();
        }
        KeyCode::Char(char) => {
            app.input_char(char);
        }
        KeyCode::Backspace => {
            app.input_backspace();
        }
        KeyCode::Delete => {
            app.input_delete();
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Home => {
            app.move_cursor_home();
        }
        KeyCode::End => {
            app.move_cursor_end();
        }
        _ => return,
    }
}

pub fn handle_paste(app: &mut App, data: String) {
    match app.current_area {
        AreaEnum::Sql => {
            let pos = app.get_cursor_position();
            app.sql_input.insert_str(pos, &data);
            app.set_cursor_position(AreaEnum::Sql, pos + data.len());
        }
        AreaEnum::Value => {
            let pos = app.get_cursor_position();
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
