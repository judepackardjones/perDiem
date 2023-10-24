use std::collections::HashMap;

use crate::types::*;
use chrono::NaiveDate as chronoDate;
use chrono::NaiveDateTime as chronoDateTime;
#[macro_export]
macro_rules! increase_date {
    ($($time_span:expr), *) => {
        todo!();
    };
}
#[macro_export]
macro_rules! decrease_date {
    ($($time_span:expr), *) => {
        todo!();
    };
}

macro_rules! impl_operators_fns {
    ($struct:ident) => {
        impl crate::types::datekindOperators for $struct {
            fn last_two_digits_year(&self) -> String {
                self.year
                    .to_string()
                    .as_str()
                    .chars()
                    .rev()
                    .take(2)
                    .map(|x| x.to_string())
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>()
            }
        }
    };
}

impl_operators_fns!(Date);
impl_operators_fns!(DateTime);

impl Date {
    pub fn to_DateTime(&self) -> DateTime {
        DateTime {
            second: 0,
            minute: 0,
            hour: 0,
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
    pub fn increase(self, length: TimeSpan) -> Date {
        let increase_date = self;
        let rollovers: HashMap<&str, i32> = HashMap::from([
            ("seconds", 60),
            ("minutes", 60),
            ("hours", 24),
            ("months", 12),
        ]);
        let month_lengths: HashMap<i32, i32> = HashMap::from([
            (1, 31),
            (2, if increase_date.isLeapYear() { 29 } else { 28 }),
            (3, 31),
            (4, 30),
            (5, 31),
            (6, 30),
            (7, 31),
            (8, 31),
            (9, 30),
            (10, 31),
            (11, 30),
            (12, 31),
        ]);
        match length {
            TimeSpan::seconds(secs) => increase_date,
            TimeSpan::minutes(mins) => todo!(),
            TimeSpan::hours(hours) => todo!(),
            TimeSpan::days(days) => todo!(),
            TimeSpan::months(months) => todo!(),
            TimeSpan::years(years) => todo!(),
        }
    }
}
impl DateTime {
    pub fn to_Date(&self) -> Date {
        Date {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
    pub fn increase(&self, length: TimeSpan) -> DateTime {
        todo!();
    }
}
