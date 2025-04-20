use crate::util;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use time::{Duration, Time};

#[derive(Serialize, Deserialize)]
pub struct TimeRange {
    pub begin: Time,
    pub end: Time,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    PerTime,
    PerHour(TimeRange),
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

impl From<&TimeRange> for Duration {
    fn from(value: &TimeRange) -> Self {
        let TimeRange { begin, end } = value;

        if begin < end {
            *end - *begin
        } else {
            *end - *begin + Duration::days(1)
        }
    }
}

impl Type {
    pub fn time_range_to_string(&self) -> String {
        match self {
            Self::PerTime => "-".to_string(),
            Self::PerHour(TimeRange { begin, end }) => format!(
                "{: >2}:{:02} - {: >2}:{:02}",
                begin.hour(),
                begin.minute(),
                end.hour(),
                end.minute()
            ),
        }
    }
}

impl From<&Type> for TypeForPickList {
    fn from(value: &Type) -> Self {
        match value {
            Type::PerTime => Self::PerTime,
            Type::PerHour(_) => Self::PerHour,
        }
    }
}

impl Config {
    pub fn pay_to_string(&self) -> String {
        format!(
            "{} {}",
            util::comma_separated(self.pay),
            TypeForPickList::from(&self.r#type)
        )
    }

    pub fn sum(&self, count: usize) -> u32 {
        match &self.r#type {
            Type::PerTime => self.pay * count as u32,
            Type::PerHour(tr) => {
                (count as f32 * self.pay as f32 * Duration::from(tr).as_seconds_f32() / 3600.0)
                    as u32
            }
        }
    }
}

pub const TYPES_FOR_PICK_LIST: [TypeForPickList; 2] =
    [TypeForPickList::PerTime, TypeForPickList::PerHour];

impl Display for TypeForPickList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PerTime => write!(f, "¥/#"),
            Self::PerHour => write!(f, "¥/h"),
        }
    }
}
