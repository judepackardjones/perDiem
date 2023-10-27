use std::collections::HashMap;
use crate::{types::*, utils::get_pos};


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
    /// Creates an instance of Date with all fields set to 1
    pub fn new() -> Date {
        Date {
            day: 1, 
            month: 1,
            year: 1,
        }
    }
    /// Creates a instance of Date with fields provided
    pub fn from(day: i8, month: i8, year: i32) -> Date {
        Date {
            day: day,
            month: month,
            year: year,
        }
    }
    /// Converts Date to DateTime and sets second, minute, and hour to 1.
    pub fn to_DateTime(&self) -> DateTime {
        DateTime {
            second: 1,
            minute: 1,
            hour: 1,
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
    fn increase(self, length: TimeSpan) -> Result<Date, &'static str> {
        let mut increase_date = self;
        let rollovers: HashMap<&str, i32> = HashMap::from([
            ("seconds", 60),
            ("minutes", 60),
            ("hours", 24),
            ("months", 12),
        ]);
        match length {
            TimeSpan::days(days) => todo!(),
            TimeSpan::months(months) => todo!(),
            TimeSpan::years(years) => {increase_date.year += years;
                Ok(increase_date) },
            _ => {Err("Invalid TimeSpan specifier")}
        }
    }
}
impl DateTime {
    /// Creates new instance of DateTime with all fields set to 1
    pub fn new() -> DateTime {
        DateTime {
            second: 1,
            minute: 1,
            hour: 1,
            day: 1, 
            month: 1,
            year: 1,
        }
    }
    /// Creates a new instance of DateTime with parameters given
    pub fn from(second: i8, minute: i8, hour: i8, day: i8, month: i8, year: i32) -> DateTime {
        DateTime {
            second: second,
            minute: minute,
            hour: hour,
            day: day,
            month: month,
            year: year,
        }
    }
    /// Returns a TimeDifference of the two dates given. Each field is always positive. 
    pub fn difference(&self, datetime: DateTime) -> TimeDifference {
        TimeDifference { seconds: get_pos(self.second.into(), datetime.second.into()), minutes: get_pos(self.minute.into(), datetime.minute.into()), hours: get_pos(self.hour.into(), datetime.hour.into()), days: get_pos(self.day.into(), datetime.day.into()), months: get_pos(self.month.into(), datetime.month.into()), years: get_pos(self.year.into(), datetime.year.into()) }
    }
    /// Converts DateTime to Date
    pub fn to_Date(&self) -> Date {
        Date {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
    fn increase(&self, length: TimeSpan) -> DateTime {
        todo!();
    }
}
