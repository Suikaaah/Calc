use crate::{failure::Failure, util};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct HourMinute {
    hour: u8,
    minute: u8,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Type {
    PerTime,
    PerHour(HourMinute),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeForPickList {
    PerTime,
    PerHour,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub r#type: Type,
    pub pay: u32,
}

pub const TYPES_FOR_PICK_LIST: [TypeForPickList; 2] =
    [TypeForPickList::PerTime, TypeForPickList::PerHour];

impl Type {
    pub fn duration_to_string(&self) -> String {
        match self {
            Self::PerTime => "-".to_string(),
            Self::PerHour(hm) => hm.to_string(),
        }
    }
}

impl From<Type> for TypeForPickList {
    fn from(value: Type) -> Self {
        match value {
            Type::PerTime => Self::PerTime,
            Type::PerHour(_) => Self::PerHour,
        }
    }
}

impl HourMinute {
    pub const fn minutes(&self) -> u16 {
        60 * self.hour as u16 + self.minute as u16
    }

    pub const fn from_hm(hour: u8, minute: u8) -> Result<Self, Failure> {
        if hour < 24 && minute < 60 {
            Ok(Self { hour, minute })
        } else {
            Err(Failure::Duration)
        }
    }
}

impl Display for HourMinute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self { hour, minute } = self;
        write!(f, "{hour: >2}:{minute:02}")
    }
}

impl Display for TypeForPickList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PerTime => write!(f, "¥/#"),
            Self::PerHour => write!(f, "¥/h"),
        }
    }
}

impl Config {
    pub fn pay_to_string(&self) -> String {
        format!(
            "{} {}",
            util::comma_separated(self.pay),
            TypeForPickList::from(self.r#type)
        )
    }

    pub fn sum(&self, count: usize) -> u32 {
        match self.r#type {
            Type::PerTime => self.pay * count as u32,
            Type::PerHour(hm) => {
                let total = (hm.minutes() as u32 * count as u32 * self.pay) as f32 / 60.0;
                total as u32
            }
        }
    }
}
