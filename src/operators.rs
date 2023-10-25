use std::collections::HashMap;
use crate::{types::*, utils::get_pos};
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
    pub fn difference(&self, datetime: DateTime) -> TimeDifference {
        TimeDifference { seconds: get_pos(self.second.into(), datetime.second.into()), minutes: get_pos(self.minute.into(), datetime.minute.into()), hours: get_pos(self.hour.into(), datetime.hour.into()), days: get_pos(self.day.into(), datetime.day.into()), months: get_pos(self.month.into(), datetime.month.into()), years: get_pos(self.year.into(), datetime.year.into()) }
    }
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
