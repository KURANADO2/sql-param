use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct App {
    pub sql_input: String,
    pub value_input: String,
    pub result: String,
    pub current_area: AreaEnum,
    pub should_exit: bool,
}

#[derive(EnumIter, PartialEq, Clone, Copy)]
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
