use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CronError {
    InvalidExpression(String),
    InvalidField(String),
    NoMatch,
}

impl fmt::Display for CronError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidExpression(message) => write!(f, "invalid cron expression: {message}"),
            Self::InvalidField(message) => write!(f, "invalid field value: {message}"),
            Self::NoMatch => write!(f, "no matching date found"),
        }
    }
}

impl Error for CronError {}

pub type CronResult<T> = Result<T, CronError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CronFieldKind {
    Minute,
    Hour,
    DayOfMonth,
    Month,
    DayOfWeek,
    Second,
    Year,
}
