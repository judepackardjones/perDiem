use ordered_hashmap::OrderedHashMap;

use crate::{types::*, utils::floor, utils::get_pos};
use std::collections::HashMap;
use std::hash::Hash;

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
    /// Returns the difference between two Dates as a TimeDifference with seconds, minutes, and hours set to 0
    pub fn difference(&self, datetime: Date) -> TimeDifference {
        TimeDifference {
            seconds: 0,
            minutes: 0,
            hours: 0,
            days: get_pos(self.day.into(), datetime.day.into()),
            months: get_pos(self.month.into(), datetime.month.into()),
            years: get_pos(self.year.into(), datetime.year.into()),
        }
    }
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
    /// Increases the Date given by the TimeSpan provided. End date is NOT included. (This would add 1 to the day.)
    pub fn increase(self, length: TimeSpan) -> Result<Date, &'static str> {
        let mut increase_date = self;
        match length {
            TimeSpan::days(days) => {
                let mut month_lengths: HashMap<i32, i32> = HashMap::from([
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
                let mut day_counter = days;
                let mut month_counter: i32 = increase_date.month as i32;
                loop {
                    *month_lengths.get_mut(&2).unwrap() = if increase_date.isLeapYear() { 29 } else { 28 };
                    // needs to be initialized each loop because leap year changes. 
                    if increase_date.day as i32 + day_counter > *month_lengths.get(&(month_counter as i32)).unwrap() as i32 {
                        day_counter -= *month_lengths.get(&(month_counter as i32)).unwrap();
                        month_counter += 1;
                        if month_counter == 13 {
                            month_counter = 1;
                        }
                        increase_date = increase_date.increase(TimeSpan::months(1)).unwrap();
                    } else {
                        increase_date.day = (day_counter + 1) as i8;
                        // if increase_date.day == 0 {
                        //     increase_date.day = 1;
                        // }
                        break;
                    }

                }
                // Find how many months are in the date
                // call itsself on months
                /* TODO: Take different approach. Take the ammount of days and subtract the day increase by the amount of days inthe current month
                then go to the next one and see if you can subtract the current month by this one and not get 0, if you can't, then set it to
                the current overflow day of the month*/ 
            Ok(increase_date)
        },
            TimeSpan::months(months) => {
                increase_date.year = increase_date.year + floor(months as f32 / 12.0);
                increase_date.month = increase_date.month + (months % 12) as i8;
                if increase_date.month > 12 {
                    increase_date.month = increase_date.month - 12;
                    increase_date.year = increase_date.year + 1;
                }
                Ok(increase_date)
            }
            TimeSpan::years(years) => {
                increase_date.year += years;
                Ok(increase_date)
            }
            _ => Err("Invalid TimeSpan specifier"),
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
        TimeDifference {
            seconds: get_pos(self.second.into(), datetime.second.into()),
            minutes: get_pos(self.minute.into(), datetime.minute.into()),
            hours: get_pos(self.hour.into(), datetime.hour.into()),
            days: get_pos(self.day.into(), datetime.day.into()),
            months: get_pos(self.month.into(), datetime.month.into()),
            years: get_pos(self.year.into(), datetime.year.into()),
        }
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
