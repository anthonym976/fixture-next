use std::fmt;

/// A calendar date. Field order matches calendar order, so the derived
/// Ord/PartialOrd impls sort dates correctly without any extra code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn new(year: i32, month: u8, day: u8) -> Result<Date, String> {
        if !(1..=12).contains(&month) {
            return Err(format!("month {} is out of range", month));
        }
        let max_day = days_in_month(year, month);
        if day == 0 || day > max_day {
            return Err(format!(
                "day {} is out of range for {:04}-{:02}",
                day, year, month
            ));
        }
        Ok(Date { year, month, day })
    }

    /// Parses a "YYYY-MM-DD" string, validating that the date actually exists
    /// (rejects things like April 31 or February 29 in a non-leap year).
    pub fn parse(s: &str) -> Result<Date, String> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return Err(format!("expected YYYY-MM-DD, got '{}'", s));
        }
        let year = parts[0]
            .parse::<i32>()
            .map_err(|_| format!("bad year in '{}'", s))?;
        let month = parts[1]
            .parse::<u8>()
            .map_err(|_| format!("bad month in '{}'", s))?;
        let day = parts[2]
            .parse::<u8>()
            .map_err(|_| format!("bad day in '{}'", s))?;
        Date::new(year, month, day)
    }

    /// Today's date in UTC, derived from the system clock without pulling in
    /// a calendar library (Howard Hinnant's civil_from_days algorithm).
    pub fn today() -> Date {
        let since_epoch = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let days = (since_epoch.as_secs() / 86_400) as i64;
        Date::from_epoch_day(days)
    }

    fn from_epoch_day(days: i64) -> Date {
        let z = days + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = (doy - (153 * mp + 2) / 5 + 1) as u8;
        let month = if mp < 10 { mp + 3 } else { mp - 9 } as u8;
        let year = if month <= 2 { y + 1 } else { y } as i32;
        Date { year, month, day }
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
