use crate::datetime::DateTime;
use crate::types::{CronError, CronResult};
use crate::CronExpression;

pub struct CronIterator {
    expression: CronExpression,
    current: DateTime,
}

impl CronIterator {
    pub fn new(expression: CronExpression, start: DateTime) -> Self {
        Self { expression, current: start }
    }

    pub fn next_match(&self) -> CronResult<DateTime> {
        self.next_after(self.current)
    }

    pub fn next_after(&self, from: DateTime) -> CronResult<DateTime> {
        let mut candidate = from.add_seconds(1);
        loop {
            if self.expression.matches(&candidate)? {
                return Ok(candidate);
            }
            candidate = candidate.add_seconds(1);
            if candidate.year > 2100 {
                return Err(CronError::NoMatch);
            }
        }
    }
}
