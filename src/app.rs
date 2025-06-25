use crate::core::replace_placeholder;
use ratatui::layout::Rect;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tui_textarea::TextArea;

pub struct App {
    pub sql_input: TextArea<'static>,
    pub value_input: TextArea<'static>,
    pub result: String,
    pub current_area: AreaEnum,
    pub should_exit: bool,
    pub area_coordinates: HashMap<AreaEnum, Rect>,
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
            sql_input: TextArea::default(),
            value_input: TextArea::default(),
            result: String::new(),
            current_area: AreaEnum::Sql,
            should_exit: false,
            area_coordinates: HashMap::new(),
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

    pub fn input_clear(&mut self) {
        match self.current_area {
            AreaEnum::Sql => {
                self.sql_input = TextArea::default();
            }
            AreaEnum::Value => {
                self.value_input = TextArea::default();
            }
            _ => return,
        }
    }

    pub fn input_char(&mut self, char: char) {
        match self.current_area {
            AreaEnum::Sql => {
                self.sql_input.insert_char(char);
            }
            AreaEnum::Value => {
                self.value_input.insert_char(char);
            }
            _ => return,
        }
    }

    pub fn input_backspace(&mut self) {
        match self.current_area {
            AreaEnum::Sql => {
                self.sql_input.delete_char();
            }
            AreaEnum::Value => {
                self.value_input.delete_char();
            }
            _ => return,
        }
    }

    pub fn get_current_textarea(&mut self) -> Option<&mut TextArea<'static>> {
        match self.current_area {
            AreaEnum::Sql => Some(&mut self.sql_input),
            AreaEnum::Value => Some(&mut self.value_input),
            _ => None,
        }
    }

    pub fn get_sql_text(&self) -> String {
        self.sql_input.lines().join("\n")
    }

    pub fn get_value_text(&self) -> String {
        self.value_input.lines().join("\n")
    }

    pub fn calculate_result(&mut self) {
        self.result = replace_placeholder(self.get_sql_text().as_str(), self.get_value_text().as_str());
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
