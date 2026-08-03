use crate::types::{CronError, CronResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl DateTime {
    pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> CronResult<Self> {
        if month == 0 || month > 12 {
            return Err(CronError::InvalidField(format!("month {month}")));
        }
        if day == 0 || day > 31 {
            return Err(CronError::InvalidField(format!("day {day}")));
        }
        if hour > 23 {
            return Err(CronError::InvalidField(format!("hour {hour}")));
        }
        if minute > 59 {
            return Err(CronError::InvalidField(format!("minute {minute}")));
        }
        if second > 59 {
            return Err(CronError::InvalidField(format!("second {second}")));
        }
        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    pub fn from_ymd_opt(year: i32, month: u32, day: u32) -> CronResult<Self> {
        Self::new(year, month, day, 0, 0, 0)
    }

    pub fn add_seconds(self, seconds: i64) -> Self {
        let mut total = self.second as i64 + seconds;
        let mut minute = self.minute as i64;
        let mut hour = self.hour as i64;
        let mut day = self.day as i64;
        let mut month = self.month as i64;
        let mut year = self.year as i64;

        while total >= 60 {
            total -= 60;
            minute += 1;
        }
        while total < 0 {
            total += 60;
            minute -= 1;
        }
        while minute >= 60 {
            minute -= 60;
            hour += 1;
        }
        while minute < 0 {
            minute += 60;
            hour -= 1;
        }
        while hour >= 24 {
            hour -= 24;
            day += 1;
        }
        while hour < 0 {
            hour += 24;
            day -= 1;
        }
        while day > days_in_month(year, month) {
            day -= days_in_month(year, month);
            month += 1;
            if month > 12 {
                month = 1;
                year += 1;
            }
        }
        while day <= 0 {
            month -= 1;
            if month < 1 {
                month = 12;
                year -= 1;
            }
            day = days_in_month(year, month);
        }
        while month > 12 {
            month -= 12;
            year += 1;
        }
        while month < 1 {
            month += 12;
            year -= 1;
        }

        Self {
            year: year as i32,
            month: month as u32,
            day: day as u32,
            hour: hour as u32,
            minute: minute as u32,
            second: total as u32,
        }
    }

    pub fn weekday(&self) -> u32 {
        let y = self.year as i64;
        let m = self.month as i64;
        let d = self.day as i64;
        let y0 = (y - (if m <= 2 { 1 } else { 0 })) as i64;
        let y = y0 as i64;
        let era = (if y >= 0 { y } else { y - 399 }) / 400;
        let yoe = (y - era * 400) as i64;
        let doy = (153 * ((m + if m > 2 { -2 } else { 9 }) % 12) + 2) / 5 + d - 1;
        let doe = (yoe * 365 + yoe / 4 - yoe / 100 + yoe / 400 + doy) as i64;
        let weekday = (era as i64 * 146097 + doe) % 7;
        ((weekday + 6) % 7) as u32
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn is_leap_year(year: i64) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}
