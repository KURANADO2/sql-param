use crate::app::{App, AreaEnum};
use crossterm::event::{
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    if key.code == KeyCode::Esc {
        app.should_exit = true;
        return;
    }

    match app.current_area {
        AreaEnum::Sql => match key.code {
            KeyCode::Tab => app.current_area = app.next_area(),
            KeyCode::BackTab => {
                app.current_area = app.prev_area();
                app.calculate_result();
            }
            _other => handle_common_key(app, key),
        },
        AreaEnum::Value => match key.code {
            KeyCode::Tab => {
                app.current_area = app.next_area();
                app.calculate_result();
            }
            KeyCode::BackTab => app.current_area = app.prev_area(),
            _other => handle_common_key(app, key),
        },
        AreaEnum::Result => match key.code {
            KeyCode::Tab => app.current_area = app.next_area(),
            KeyCode::BackTab => app.current_area = app.prev_area(),
            _ => return,
        },
    }
}

fn handle_common_key(app: &mut App, key: KeyEvent) {
    match key.code {
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
        KeyCode::Enter => {
            if let Some(textarea) = app.get_current_input() {
                textarea.insert_newline();
            }
        }
        KeyCode::Up => {
            if let Some(textarea) = app.get_current_input() {
                textarea.move_cursor(tui_textarea::CursorMove::Up);
            }
        }
        KeyCode::Down => {
            if let Some(textarea) = app.get_current_input() {
                textarea.move_cursor(tui_textarea::CursorMove::Down);
            }
        }
        KeyCode::Left => {
            if let Some(textarea) = app.get_current_input() {
                textarea.move_cursor(tui_textarea::CursorMove::Back);
            }
        }
        KeyCode::Right => {
            if let Some(textarea) = app.get_current_input() {
                textarea.move_cursor(tui_textarea::CursorMove::Forward);
            }
        }
        KeyCode::Home => {
            if let Some(textarea) = app.get_current_input() {
                textarea.move_cursor(tui_textarea::CursorMove::Head);
            }
        }
        KeyCode::End => {
            if let Some(textarea) = app.get_current_input() {
                textarea.move_cursor(tui_textarea::CursorMove::End);
            }
        }
        _ => return,
    }
}

pub fn handle_paste(app: &mut App, data: String) {
    match app.current_area {
        AreaEnum::Sql => {
            app.sql_input.insert_str(&data);
        }
        AreaEnum::Value => {
            app.value_input.insert_str(&data);
        }
        _ => return,
    }
}

pub fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
        if let Some(clicked_area) = app.get_area_by_coordinate(mouse.column, mouse.row) {
            app.current_area = clicked_area;
            if clicked_area == AreaEnum::Result {
                app.calculate_result();
            }
        }
    }
}
