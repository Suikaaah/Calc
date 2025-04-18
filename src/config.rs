use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    PerTime,
    PerHour,
}

#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub r#type: Type,
    pub pay: u32,
}

impl Config {
    pub fn pay_to_string(&self) -> String {
        let unit = match self.r#type {
            Type::PerTime => "¥/time",
            Type::PerHour => "¥/h",
        };

        format!("{} [{unit}]", self.pay)
    }
}

pub const TYPES: [Type; 2] = [Type::PerTime, Type::PerHour];

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
