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

    pub fn cursor_position(&self, area_enum: AreaEnum) -> usize {
        match area_enum {
            AreaEnum::Sql => self.sql_cursor_position,
            AreaEnum::Value => self.value_cursor_position,
            AreaEnum::Result => 0,
        }
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
