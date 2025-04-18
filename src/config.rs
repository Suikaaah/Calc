use std::fmt::{Display, Formatter};
use time::{Duration, Time};

pub struct TimeRange {
    pub begin: Time,
    pub end: Time,
}

pub enum Type {
    PerTime,
    PerHour(TimeRange),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeForPickList {
    PerTime,
    PerHour,
}

pub struct Config {
    pub name: String,
    pub r#type: Type,
    pub pay: u32,
}

impl From<TimeRange> for Duration {
    fn from(value: TimeRange) -> Self {
        let TimeRange { begin, end } = value;

        if begin < end {
            end - begin
        } else {
            end - begin + Duration::days(1)
        }
    }
}

impl Type {
    pub fn time_range_to_string(&self) -> String {
        match self {
            Self::PerTime => "-".to_string(),
            Self::PerHour(TimeRange { begin, end }) => format!(
                "{}:{:02} - {}:{:02}",
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
        format!("{} [{}]", self.pay, TypeForPickList::from(&self.r#type))
    }
}

pub const TYPES_FOR_PICK_LIST: [TypeForPickList; 2] =
    [TypeForPickList::PerTime, TypeForPickList::PerHour];

impl Display for TypeForPickList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PerTime => write!(f, "¥/time"),
            Self::PerHour => write!(f, "¥/h"),
        }
    }
}
