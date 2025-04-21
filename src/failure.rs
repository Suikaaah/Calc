use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone, Copy)]
pub enum Failure {
    Cell(usize),
    Offset,
    Year,
    Load,
    Save,
    SaveEmptyName,
    Pay,
    FileRemove,
    DurationParse,
    Duration,
    Date,
}

impl Display for Failure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cell(i) => write!(f, "cells[{i}] was referenced (supposed to be unreachable)"),
            Self::Offset => write!(
                f,
                "Offset must be a number (u8) (unreachable due to unwrap_or(N/A))"
            ),
            Self::Year => write!(
                f,
                "Year must be a number (i32) (unreachable due to unwrap_or(N/A))"
            ),
            Self::Load => write!(f, "Load failed"),
            Self::Save => write!(f, "Save failed"),
            Self::SaveEmptyName => write!(f, "Cannot save without name"),
            Self::Pay => write!(f, "Pay must be a number (u32)"),
            Self::FileRemove => write!(f, "Delete failed"),
            Self::DurationParse => write!(f, "Duration must consist of numbers (u8)"),
            Self::Duration => write!(f, "Invalid duration"),
            Self::Date => write!(f, "Invalid date"),
        }
    }
}

impl Error for Failure {}
