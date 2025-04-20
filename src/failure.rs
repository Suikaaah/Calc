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
    Pay,
    DurationParse,
    Duration,
    Date,
}

impl Display for Failure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cell(i) => write!(f, "cells[{i}] was referenced (unreachable)"),
            Self::Offset => write!(f, "Offset must be a number (u8) (unreachable)"),
            Self::Year => write!(f, "Year must be a number (i32) (unreachable)"),
            Self::Load => write!(f, "Load failed"),
            Self::Save => write!(f, "Save failed"),
            Self::Pay => write!(f, "Pay must be a number (u32)"),
            Self::DurationParse => write!(f, "Duration must consist of numbers (u8)"),
            Self::Duration => write!(f, "Invalid duration"),
            Self::Date => write!(f, "Invalid date"),
        }
    }
}

impl Error for Failure {}
