use crate::{types::*, utils::floor, utils::get_pos};
use std::collections::HashMap;

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
    /// Mutates the receiver Date by the TimeSpan sepcified and returns a Result.
    pub fn increase(&mut self, length: TimeSpan) -> Result<(), &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        *self = self.increase_as_new(length.clone()).unwrap();
        Ok(())
    }
    // TODO: End of Experimental increase
    /// Returns the difference between two Dates as a TimeDifference with seconds, minutes, and hours set to 0
    pub fn difference(&self, datetime: &Date) -> TimeDifference {
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
        /// Creates 
        pub fn increase_as_new(&self, length: TimeSpan) -> Result<Date, &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        let mut increase_date = self.clone();
        match length {
            TimeSpan::days(days) => {
                let initial_day = increase_date.day;
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
                        day_counter -= *month_lengths.get(&(month_counter as i32)).unwrap_or(&0);
                        month_counter += 1;
                        if month_counter == 13 {
                            month_counter = 1;
                        }
                        increase_date = increase_date.increase_as_new(TimeSpan::months(1)).unwrap();
                    } else {
                        increase_date.day = (day_counter + (if initial_day == 1 { 1 } else { (initial_day) as i32})) as i8;
                        break;
                    }

                }
        },
            TimeSpan::months(months) => {
                increase_date.year = increase_date.year + floor(months as f32 / 12.0);
                increase_date.month = increase_date.month + (months % 12) as i8;
                if increase_date.month > 12 {
                    increase_date.month = increase_date.month - 12;
                    increase_date.year = increase_date.year + 1;
                }
            }
            TimeSpan::years(years) => {
                increase_date.year += years;
                increase_date.day = if !increase_date.isLeapYear() && increase_date.month == 2 && increase_date.day == 29 { 28 } else { increase_date.day};
            }
            _ => {return Err("Invalid TimeSpan specifier, make sure that you are using a valid TimeSpan for the Date's increase method");},
        }
        let final_date: Date = Date {
            day: increase_date.day,
            month: increase_date.month,
            year: increase_date.year,
        };
        Ok(final_date)
    }
    /// Decreases Date by given TimeSpan parameter. (Unfinished)
    pub fn decrease(self, length: TimeSpan) -> Result<Date, &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        let mut decrease_date = self;
        match length {
            TimeSpan::seconds(seconds) => {
                
            },
            TimeSpan::minutes(minutes) => {
                
            },
            TimeSpan::hours(hours) => {
                
            },
            TimeSpan::days(days) => {
                let initial_day = decrease_date.day as i32;
                let mut month_lengths: HashMap<i32, i32> = HashMap::from([
                    (1, 31),
                    (2, if decrease_date.isLeapYear() { 29 } else { 28 }),
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
                let mut month_skips = 0;
                loop {
                    if initial_day - days > 0 {
                        decrease_date.day -= days as i8;
                        break;
                    } else {
                        month_skips += 1;
                        if initial_day - days == 0 {
                            decrease_date.day = *month_lengths.get(&(decrease_date.month as i32)).unwrap() as i8;
                            break;
                        } else {
                            
                        }
                    }
                }
            },
            TimeSpan::months(months) => {
                decrease_date.year = decrease_date.year - floor(months as f32 / 12.0);
                decrease_date.month = decrease_date.month - (months % 12) as i8;
                if decrease_date.month <= 0 {
                    decrease_date.month = decrease_date.month + 12;
                    decrease_date.year = decrease_date.year - 1;
                }
            },
            TimeSpan::years(years) => {
                decrease_date.year -= years;
                decrease_date.day = if !decrease_date.isLeapYear() && decrease_date.month == 2 && decrease_date.day == 29 { 28 } else { decrease_date.day };
            },
        }
        Ok(decrease_date)
    }
}

impl DateTime {
    pub fn increase(&mut self, length: TimeSpan) -> Result<(), &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        *self = self.increase_as_new(length.clone()).unwrap();
        Ok(())
    }
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
    pub fn difference(&self, datetime: &DateTime) -> TimeDifference {
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
    /// Increases the given DateTime by the TimeSpan specified
    pub fn increase_as_new(&self, length: TimeSpan) -> Result<DateTime, &'static str> {
        let mut increase_date = self.clone();
        match length {
            TimeSpan::seconds(seconds) => {
                let second_floor =
                    floor(((increase_date.second as i32 + seconds) as f32) / 60.0) as i8;
                increase_date = increase_date
                    .increase_as_new(TimeSpan::minutes(second_floor as i32).into())
                    .unwrap();
                increase_date.second = increase_date.second + (seconds % 60) as i8;
                if increase_date.second > 59 {
                    increase_date.second = increase_date.second - 60;
                }
            }
            TimeSpan::minutes(minutes) => {
                let minute_floor =
                    floor(((increase_date.minute as i32 + minutes) as f32) / 60.0) as i8;
                increase_date = increase_date
                    .increase_as_new(TimeSpan::hours(minute_floor as i32).into())
                    .unwrap();
                increase_date.minute = increase_date.minute + (minutes % 60) as i8;
                if increase_date.minute > 59 {
                    increase_date.minute = increase_date.minute - 60;
                }
            }
            TimeSpan::hours(hours) => {
                let hour_floor = floor(((increase_date.hour as i32 + hours) as f32) / 24.0) as i8;
                increase_date = increase_date
                    .increase_as_new(TimeSpan::days(hour_floor as i32))
                    .unwrap();
                increase_date.hour = increase_date.hour + (hours % 24) as i8;
                if increase_date.hour > 23 {
                    increase_date.hour = increase_date.hour - 24;
                }

            }
            TimeSpan::days(days) => {
                let initial_day = increase_date.day;
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
                        day_counter -= *month_lengths.get(&(month_counter as i32)).unwrap_or(&0);
                        month_counter += 1;
                        if month_counter == 13 {
                            month_counter = 1;
                        }
                        increase_date = increase_date.increase_as_new(TimeSpan::months(1)).unwrap();
                    } else {
                        increase_date.day = (day_counter + (if initial_day == 1 { 1 } else { (initial_day) as i32})) as i8;
                        break;
                    }

                }
            }
            TimeSpan::months(months) => {
                increase_date.year = increase_date.year + floor(months as f32 / 12.0);
                increase_date.month = increase_date.month + (months % 12) as i8;
                if increase_date.month > 12 {
                    increase_date.month = increase_date.month - 12;
                    increase_date.year = increase_date.year + 1;
                }
            }
            TimeSpan::years(years) => {
                increase_date.year += years;
                increase_date.day = if !increase_date.isLeapYear() && increase_date.month == 2 && increase_date.day == 29 { 28 } else { increase_date.day};
            }
        }
        Ok(increase_date)
    }
}

impl Clone for Date {
    fn clone(&self) -> Date {
        Date {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}
impl Clone for DateTime {
    fn clone(&self) -> DateTime {
        DateTime {
            second: self.second,
            minute: self.minute,
            hour: self.hour,
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}
impl Clone for TimeSpan {
    fn clone(&self) -> TimeSpan {
        match self {
            TimeSpan::seconds(seconds) => {
                return TimeSpan::seconds(*seconds);
            }
            TimeSpan::minutes(minutes) => {
                return TimeSpan::minutes(*minutes);
            }
            TimeSpan::hours(hours) => {
                return TimeSpan::hours(*hours);
            }
            TimeSpan::days(days) => {
                return TimeSpan::days(*days);
            }
            TimeSpan::months(months) => {
                return TimeSpan::months(*months);
            }
            TimeSpan::years(years) => {
                return TimeSpan::years(*years);
            }
        }
    }
}
