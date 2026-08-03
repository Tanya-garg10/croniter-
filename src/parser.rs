use std::collections::BTreeSet;

use crate::datetime::DateTime;
use crate::types::{CronError, CronFieldKind, CronResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CronFieldValue {
    pub kind: CronFieldKind,
    pub values: Vec<u32>,
    pub wildcard: bool,
}

#[derive(Debug, Clone)]
pub struct CronExpression {
    pub fields: Vec<CronFieldValue>,
    pub day_or: bool,
}

impl CronExpression {
    pub fn parse(expression: &str) -> CronResult<Self> {
        let parts: Vec<&str> = expression.split_whitespace().collect();
        let mut fields = Vec::new();

        let kinds: Vec<CronFieldKind> = match parts.len() {
            5 => vec![
                CronFieldKind::Minute,
                CronFieldKind::Hour,
                CronFieldKind::DayOfMonth,
                CronFieldKind::Month,
                CronFieldKind::DayOfWeek,
            ],
            6 => vec![
                CronFieldKind::Minute,
                CronFieldKind::Hour,
                CronFieldKind::DayOfMonth,
                CronFieldKind::Month,
                CronFieldKind::DayOfWeek,
                CronFieldKind::Second,
            ],
            7 => vec![
                CronFieldKind::Minute,
                CronFieldKind::Hour,
                CronFieldKind::DayOfMonth,
                CronFieldKind::Month,
                CronFieldKind::DayOfWeek,
                CronFieldKind::Second,
                CronFieldKind::Year,
            ],
            _ => return Err(CronError::InvalidExpression(expression.to_string())),
        };

        for (index, part) in parts.iter().enumerate() {
            let kind = kinds[index];
            let value = parse_field(part, kind)?;
            fields.push(value);
        }

        if parts.len() == 5 {
            fields.push(CronFieldValue {
                kind: CronFieldKind::Second,
                values: vec![0],
                wildcard: false,
            });
        }

        Ok(Self { fields, day_or: true })
    }

    pub fn matches(&self, value: &DateTime) -> CronResult<bool> {
        let minute = value.minute as u32;
        let hour = value.hour as u32;
        let day = value.day as u32;
        let month = value.month as u32;
        let weekday = value.weekday();
        let second = value.second as u32;
        let year = value.year as u32;

        let minute_ok = self.fields[0].values.contains(&minute);
        let hour_ok = self.fields[1].values.contains(&hour);
        let month_ok = self.fields[3].values.contains(&month);

        let dom_field = &self.fields[2];
        let dow_field = &self.fields[4];
        let dom_ok = dom_field.values.contains(&day);
        let dow_ok = dow_field.values.contains(&weekday);
        let day_ok = if dom_field.wildcard && dow_field.wildcard {
            true
        } else if dom_field.wildcard {
            dow_ok
        } else if dow_field.wildcard {
            dom_ok
        } else if self.day_or {
            dom_ok || dow_ok
        } else {
            dom_ok && dow_ok
        };

        let second_ok = if self.fields.len() > 5 {
            self.fields[5].values.contains(&second)
        } else {
            true
        };
        let year_ok = if self.fields.len() > 6 {
            self.fields[6].values.contains(&year)
        } else {
            true
        };

        Ok(minute_ok && hour_ok && month_ok && day_ok && second_ok && year_ok)
    }
}

fn parse_field(expr: &str, kind: CronFieldKind) -> CronResult<CronFieldValue> {
    let wildcard = expr == "*";
    if wildcard {
        return Ok(CronFieldValue {
            kind,
            values: allowed_values(kind),
            wildcard: true,
        });
    }

    let mut values = BTreeSet::new();
    for item in expr.split(',') {
        let item = item.trim();
        if item.is_empty() {
            return Err(CronError::InvalidField(expr.to_string()));
        }
        if let Some((base, step)) = item.split_once('/') {
            let step = step.parse::<u32>().map_err(|_| CronError::InvalidField(expr.to_string()))?;
            if step == 0 {
                return Err(CronError::InvalidField(expr.to_string()));
            }
            let (start, end) = if base == "*" {
                (field_min(kind), field_max(kind))
            } else if let Some((start, end)) = base.split_once('-') {
                (parse_value(start, kind)?, parse_value(end, kind)?)
            } else {
                let value = parse_value(base, kind)?;
                (value, value)
            };
            for value in (start..=end).step_by(step as usize) {
                values.insert(value);
            }
        } else if let Some((start, end)) = item.split_once('-') {
            let start = parse_value(start, kind)?;
            let end = parse_value(end, kind)?;
            if end < start {
                return Err(CronError::InvalidField(expr.to_string()));
            }
            for value in start..=end {
                values.insert(value);
            }
        } else {
            values.insert(parse_value(item, kind)?);
        }
    }

    Ok(CronFieldValue {
        kind,
        values: values.into_iter().collect(),
        wildcard: false,
    })
}

fn allowed_values(kind: CronFieldKind) -> Vec<u32> {
    match kind {
        CronFieldKind::Minute => (0..60).collect(),
        CronFieldKind::Hour => (0..24).collect(),
        CronFieldKind::DayOfMonth => (1..=31).collect(),
        CronFieldKind::Month => (1..=12).collect(),
        CronFieldKind::DayOfWeek => (0..=6).collect(),
        CronFieldKind::Second => (0..60).collect(),
        CronFieldKind::Year => (1970..=2099).collect(),
    }
}

fn field_min(kind: CronFieldKind) -> u32 {
    match kind {
        CronFieldKind::Minute | CronFieldKind::Hour | CronFieldKind::Second => 0,
        CronFieldKind::DayOfMonth => 1,
        CronFieldKind::Month => 1,
        CronFieldKind::DayOfWeek => 0,
        CronFieldKind::Year => 1970,
    }
}

fn field_max(kind: CronFieldKind) -> u32 {
    match kind {
        CronFieldKind::Minute | CronFieldKind::Second => 59,
        CronFieldKind::Hour => 23,
        CronFieldKind::DayOfMonth => 31,
        CronFieldKind::Month => 12,
        CronFieldKind::DayOfWeek => 6,
        CronFieldKind::Year => 2099,
    }
}

fn parse_value(expr: &str, kind: CronFieldKind) -> CronResult<u32> {
    let expr = expr.trim().to_lowercase();
    let value = match kind {
        CronFieldKind::Month => match expr.as_str() {
            "jan" => 1,
            "feb" => 2,
            "mar" => 3,
            "apr" => 4,
            "may" => 5,
            "jun" => 6,
            "jul" => 7,
            "aug" => 8,
            "sep" => 9,
            "oct" => 10,
            "nov" => 11,
            "dec" => 12,
            _ => expr.parse::<u32>().map_err(|_| CronError::InvalidField(expr.to_string()))?,
        },
        CronFieldKind::DayOfWeek => match expr.as_str() {
            "sun" | "sunday" => 0,
            "mon" | "monday" => 1,
            "tue" | "tuesday" => 2,
            "wed" | "wednesday" => 3,
            "thu" | "thursday" => 4,
            "fri" | "friday" => 5,
            "sat" | "saturday" => 6,
            _ => expr.parse::<u32>().map_err(|_| CronError::InvalidField(expr.to_string()))?,
        },
        _ => expr.parse::<u32>().map_err(|_| CronError::InvalidField(expr.to_string()))?,
    };

    let lower = field_min(kind);
    let upper = field_max(kind);
    if kind == CronFieldKind::DayOfWeek && value == 7 {
        return Ok(0);
    }
    if value < lower || value > upper {
        return Err(CronError::InvalidField(expr.to_string()));
    }
    Ok(value)
}
