use ratatui::layout::Rect;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct App {
    pub sql_input: String,
    pub value_input: String,
    pub result: String,
    pub current_area: AreaEnum,
    pub should_exit: bool,
    pub area_coordinates: HashMap<AreaEnum, Rect>,
    pub sql_cursor_position: usize,
    pub value_cursor_position: usize,
}

#[derive(EnumIter, PartialEq, Clone, Copy, Hash, Eq)]
pub enum AreaEnum {
    Sql,
    Value,
    Result,
}

impl App {
    pub fn new() -> App {
        App {
            sql_input: String::new(),
            value_input: String::new(),
            result: String::new(),
            current_area: AreaEnum::Sql,
            should_exit: false,
            area_coordinates: HashMap::new(),
            sql_cursor_position: 0,
            value_cursor_position: 0,
        }
    }

    pub fn next_area(&mut self) -> AreaEnum {
        let all: Vec<_> = AreaEnum::iter().collect();
        let index = all.iter().position(|x| x == &self.current_area).unwrap();
        let next_index = (index + 1) % all.len();
        all[next_index]
    }

    pub fn prev_area(&mut self) -> AreaEnum {
        let all: Vec<_> = AreaEnum::iter().collect();
        let index = all.iter().position(|x| x == &self.current_area).unwrap();
        let prev_index = if index == 0 { all.len() - 1 } else { index - 1 };
        all[prev_index]
    }

    pub fn content(&mut self, area_enum: AreaEnum) -> String {
        match area_enum {
            AreaEnum::Sql => self.sql_input.clone(),
            AreaEnum::Value => self.value_input.clone(),
            AreaEnum::Result => self.result.clone(),
        }
    }

    pub fn set_area_coordinate(&mut self, area: AreaEnum, rect: Rect) {
        self.area_coordinates.insert(area, rect);
    }

    pub fn get_area_by_coordinate(&self, x: u16, y: u16) -> Option<AreaEnum> {
        for (area, rect) in &self.area_coordinates {
            if x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height {
                return Some(*area);
            }
        }
        None
    }

    pub fn set_cursor_position(&mut self, area_enum: AreaEnum, position: usize) {
        match area_enum {
            AreaEnum::Sql => {
                self.sql_cursor_position = position.min(self.sql_input.len());
            }
            AreaEnum::Value => {
                self.value_cursor_position = position.min(self.value_input.len());
            }
            AreaEnum::Result => {}
        }
    }

    pub fn get_cursor_position(&self) -> usize {
        match self.current_area {
            AreaEnum::Sql => self.sql_cursor_position,
            AreaEnum::Value => self.value_cursor_position,
            AreaEnum::Result => 0,
        }
    }

    pub fn move_cursor_left(&mut self) {
        self.move_cursor_left_with_pos(self.current_area, self.get_cursor_position());
    }

    pub fn move_cursor_left_with_pos(&mut self, area_enum: AreaEnum, pos: usize) {
        if pos > 0 {
            self.set_cursor_position(area_enum, pos - 1)
        }
    }

    pub fn move_cursor_right(&mut self) {
        self.move_cursor_right_with_pos(self.current_area, self.get_cursor_position());
    }

    pub fn move_cursor_right_with_pos(&mut self, area_enum: AreaEnum, pos: usize) {
        match area_enum {
            AreaEnum::Sql => {
                if pos < self.sql_input.len() {
                    self.set_cursor_position(area_enum, pos + 1);
                }
            }
            AreaEnum::Value => {
                if pos < self.value_input.len() {
                    self.set_cursor_position(area_enum, pos + 1);
                }
            }
            _ => return,
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.set_cursor_position(self.current_area, 0);
    }

    pub fn move_cursor_end(&mut self) {
        match self.current_area {
            AreaEnum::Sql => {
                self.set_cursor_position(self.current_area, self.sql_input.len());
            }
            AreaEnum::Value => {
                self.set_cursor_position(self.current_area, self.value_input.len());
            }
            _ => return,
        }
    }

    pub fn input_clear(&mut self) {
        self.set_cursor_position(self.current_area, 0);
        match self.current_area {
            AreaEnum::Sql => {
                self.sql_input.clear();
            }
            AreaEnum::Value => {
                self.value_input.clear();
            }
            _ => return,
        }
    }

    pub fn input_char(&mut self, char: char) {
        let pos = self.get_cursor_position();
        match self.current_area {
            AreaEnum::Sql => {
                self.sql_input.insert(pos, char);
            }
            AreaEnum::Value => {
                self.value_input.insert(pos, char);
            }
            _ => return,
        }
        self.move_cursor_right_with_pos(self.current_area, pos);
    }

    pub fn input_backspace(&mut self) {
        let pos = self.get_cursor_position();
        if pos <= 0 {
            return;
        }
        match self.current_area {
            AreaEnum::Sql => {
                self.sql_input.remove(pos - 1);
            }
            AreaEnum::Value => {
                self.value_input.remove(pos - 1);
            }
            _ => return,
        }
        self.move_cursor_left_with_pos(self.current_area, pos);
    }

    pub fn input_delete(&mut self) {
        let pos = self.get_cursor_position();
        // The cursor does not need to move.
        match self.current_area {
            AreaEnum::Sql => {
                if pos < self.sql_input.len() {
                    self.sql_input.remove(pos);
                }
            }
            AreaEnum::Value => {
                if pos < self.value_input.len() {
                    self.value_input.remove(pos);
                }
            }
            _ => return,
        }
    }

    pub fn show_cursor(&mut self, area_enum: AreaEnum) -> bool {
        self.current_area == area_enum
            && (area_enum == AreaEnum::Sql || area_enum == AreaEnum::Value)
    }
}

impl AreaEnum {
    pub fn title(&self) -> &str {
        match self {
            AreaEnum::Sql => "Sql with placeholders",
            AreaEnum::Value => "Value",
            AreaEnum::Result => "Result",
        }
    }
}
