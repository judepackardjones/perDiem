use crate::evals::{days_in_month, isLeapYear};
use crate::{types::*, utils::floor, utils::get_pos};
use std::collections::HashMap;

macro_rules! impl_operators_fns {
    ($struct:ident) => {
        impl crate::types::datekindOperators for $struct {
            /// Returns the last two digits of the given year
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
    /// Converts given Date to OrdinalDate
    pub fn to_OrdinalDate(&self) -> Result<OrdinalDate, &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        let mut day_counter: i16 = self.day as i16;
        let mut month_counter: u8 = 1;
        while month_counter < self.month {
            day_counter += days_in_month(month_counter as u8, self.year) as i16;
            month_counter += 1;
        }
        Ok(OrdinalDate {
            day: day_counter as u16,
            year: self.year,
        })
    }
    /// Changes Date to be start of year
    pub fn start_of_year(&mut self) {
        *self = Date {
            day: 1,
            month: 1,
            year: self.year,
        };
    }
    /// Changes Date to be start of current month
    pub fn start_of_month(&mut self) {
        *self = Date {
            day: 1,
            month: self.month,
            year: self.year,
        };
    }
    /// Changes Date to be end of current year
    pub fn end_of_year(&mut self) {
        *self = Date {
            day: 31,
            month: 12,
            year: self.year,
        };
    }
    /// Changes Date to be end of current month
    pub fn end_of_month(&mut self) {
        let month_lengths: HashMap<i32, i32> = HashMap::from([
            (1, 31),
            (2, if self.isLeapYear() { 29 } else { 28 }),
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
        *self = Date {
            day: *month_lengths.get(&(self.month as i32)).unwrap() as u8,
            month: self.month,
            year: self.year,
        }
    }
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
    pub fn from(day: u8, month: u8, year: i32) -> Result<Date, &'static str> {
        let date = Date {
            day: day,
            month: month,
            year: year,
        };
        if !date.is_valid() {
            return Err("Invalid parameters, make sure every field is within range");
        }
        Ok(date)
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
    /// Decreases given Date using conversion to ordinal for days. Much better at larger increases than other, but much poorer at smaller increases.*Difference is only for TimeSpan::days
    pub fn decrease_ordinally(&self, length: TimeSpan) -> Result<Date, &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        let mut decrease_date = self.clone();
        match length {
            TimeSpan::days(days) => {
                decrease_date = decrease_date.to_OrdinalDate().unwrap().decrease_by_days(days).unwrap().to_Date().unwrap();
            }
            TimeSpan::months(months) => {
                decrease_date.year = decrease_date.year - floor(months as f32 / 12.0);
                decrease_date.month = decrease_date.month - (months % 12) as u8;
                if decrease_date.month <= 0 {
                    decrease_date.month = decrease_date.month + 12;
                    decrease_date.year = decrease_date.year - 1;
                }
            }
            TimeSpan::years(years) => {
                decrease_date.year -= years;
                decrease_date.day = if !decrease_date.isLeapYear()
                    && decrease_date.month == 2
                    && decrease_date.day == 29
                {
                    28
                } else {
                    decrease_date.day
                };
            }
            _ => {
                return Err("Invalid TimeSpan specifier, make sure that you are using a valid TimeSpan for Date");
            }
        }
        Ok(decrease_date)
    }
    /// Increases given Date using conversion to ordinal for days. Much better at larger increases than other, but much poorer at smaller increases.*Difference is only for TimeSpan::days
    pub fn increase_ordinally(&self, length: TimeSpan) -> Result<Date, &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        let mut increase_date = self.clone();
        match length {
            TimeSpan::days(days) => {
                increase_date = increase_date.to_OrdinalDate().unwrap().increase_by_days(days).unwrap().to_Date().unwrap();
            },
            TimeSpan::months(months) => {
                increase_date.year = increase_date.year + floor(months as f32 / 12.0);
                increase_date.month = increase_date.month + (months % 12) as u8;
                if increase_date.month > 12 {
                    increase_date.month = increase_date.month - 12;
                    increase_date.year = increase_date.year + 1;
                }
            },
            TimeSpan::years(years) => {
                increase_date.year += years;
                increase_date.day = if !increase_date.isLeapYear()
                    && increase_date.month == 2
                    && increase_date.day == 29
                {
                    28
                } else {
                    increase_date.day
                };
            },
            _ => {
                return Err("Invalid TimeSpan specifier, make sure that you are using a valid TimeSpan for the Date's increase method");
            
            }
        }
        Ok(increase_date)
    }
    /// Creates a new instance of Date that has been incremented by given TimeSpan parameter
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
                    *month_lengths.get_mut(&2).unwrap() =
                        if increase_date.isLeapYear() { 29 } else { 28 };
                    // needs to be initialized each loop because leap year changes.
                    if increase_date.day as i32 + day_counter
                        > *month_lengths.get(&(month_counter as i32)).unwrap() as i32
                    {
                        day_counter -= *month_lengths.get(&(month_counter as i32)).unwrap_or(&0);
                        month_counter += 1;
                        if month_counter == 13 {
                            month_counter = 1;
                        }
                        increase_date = increase_date.increase_as_new(TimeSpan::months(1)).unwrap();
                    } else {
                        increase_date.day = (day_counter
                            + (if initial_day == 1 {
                                1
                            } else {
                                (initial_day) as i32
                            })) as u8;
                        break;
                    }
                }
            }
            TimeSpan::months(months) => {
                increase_date.year = increase_date.year + floor(months as f32 / 12.0);
                increase_date.month = increase_date.month + (months % 12) as u8;
                if increase_date.month > 12 {
                    increase_date.month = increase_date.month - 12;
                    increase_date.year = increase_date.year + 1;
                }
            }
            TimeSpan::years(years) => {
                increase_date.year += years;
                increase_date.day = if !increase_date.isLeapYear()
                    && increase_date.month == 2
                    && increase_date.day == 29
                {
                    28
                } else {
                    increase_date.day
                };
            }
            _ => {
                return Err("Invalid TimeSpan specifier, make sure that you are using a valid TimeSpan for the Date's increase method");
            }
        }
        let final_date: Date = Date {
            day: increase_date.day,
            month: increase_date.month,
            year: increase_date.year,
        };
        Ok(final_date)
    }
    /// Decreases Date by given TimeSpan parameter. (Unfinished)
    pub fn decrease_as_new(&self, length: TimeSpan) -> Result<Date, &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        let mut decrease_date = self.clone();
        match length {
            TimeSpan::days(days) => {
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
                let mut day_counter = decrease_date.day as i32;
                day_counter -= days as i32;
                if day_counter as i32 > 0 {
                    decrease_date.day = day_counter as u8;
                } else {
                    while decrease_date.day < 0 {
                        decrease_date = self.decrease_as_new(TimeSpan::months(1)).unwrap();
                        day_counter += decrease_date.days_in_month() as i32;
                        println!("Current day counter: {}", day_counter);
                    }
                    day_counter += decrease_date.days_in_month() as i32;
                }
                decrease_date.day = day_counter as u8;
            }
            TimeSpan::months(months) => {
                decrease_date.year = decrease_date.year - floor(months as f32 / 12.0);
                decrease_date.month = decrease_date.month - (months % 12) as u8;
                if decrease_date.month <= 0 {
                    decrease_date.month = decrease_date.month + 12;
                    decrease_date.year = decrease_date.year - 1;
                }
            }
            TimeSpan::years(years) => {
                decrease_date.year -= years;
                decrease_date.day = if !decrease_date.isLeapYear()
                    && decrease_date.month == 2
                    && decrease_date.day == 29
                {
                    28
                } else {
                    decrease_date.day
                };
            }
            _ => {
                return Err("Invalid TimeSpan specifier, make sure that you are using a valid TimeSpan for Date");
            }
        }
        Ok(decrease_date)
    }
}

impl DateTime {
    /// Converts given DateTime to OrdinalDate
    pub fn to_OrdinalDate(&self) -> Result<OrdinalDate, &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        let mut day_counter: i16 = self.day as i16;
        let mut month_counter: u8 = 1;
        while month_counter < self.month {
            day_counter += days_in_month(month_counter as u8, self.year) as i16;
            month_counter += 1;
        }
        Ok(OrdinalDate {
            day: day_counter as u16,
            year: self.year,
        })
    }
    /// Changes DateTime to be start of year
    pub fn start_of_year(&mut self) {
        *self = DateTime {
            day: 1,
            month: 1,
            year: self.year,
            second: 0,
            minute: 0,
            hour: 0,
        };
    }
    /// Changes DateTime to be start of current month
    pub fn start_of_month(&mut self) {
        *self = DateTime {
            day: 1,
            month: self.month,
            year: self.year,
            second: 0,
            minute: 0,
            hour: 0,
        };
    }
    /// Changes DateTime to be end of current year
    pub fn end_of_year(&mut self) {
        *self = DateTime {
            day: 31,
            month: 12,
            year: self.year,
            second: 59,
            minute: 59,
            hour: 23,
        };
    }
    /// Changes DateTime to be end of current month
    pub fn end_of_month(&mut self) {
        let month_lengths: HashMap<i32, i32> = HashMap::from([
            (1, 31),
            (2, if self.isLeapYear() { 29 } else { 28 }),
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
        *self = DateTime {
            day: *month_lengths.get(&(self.month as i32)).unwrap() as u8,
            month: self.month,
            year: self.year,
            second: 59,
            minute: 59,
            hour: 23,
        }
    }
    /// Mutates the receiver DateTime by the TimeSpan sepcified and returns a Result.
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
            second: 0,
            minute: 0,
            hour: 0,
            day: 1,
            month: 1,
            year: 1,
        }
    }
    /// Creates a new instance of DateTime with parameters given
    pub fn from(
        second: u8,
        minute: u8,
        hour: u8,
        day: u8,
        month: u8,
        year: i32,
    ) -> Result<DateTime, &'static str> {
        let datetime = DateTime {
            second: second,
            minute: minute,
            hour: hour,
            day: day,
            month: month,
            year: year,
        };
        if !datetime.is_valid() {
            return Err("Invalid parameters, make sure everything is within range");
        }
        Ok(datetime)
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
                    floor(((increase_date.second as i32 + seconds) as f32) / 60.0) as u8;
                increase_date = increase_date
                    .increase_as_new(TimeSpan::minutes(second_floor as i32).into())
                    .unwrap();
                increase_date.second = increase_date.second + (seconds % 60) as u8;
                if increase_date.second > 59 {
                    increase_date.second = increase_date.second - 60;
                }
            }
            TimeSpan::minutes(minutes) => {
                let minute_floor =
                    floor(((increase_date.minute as i32 + minutes) as f32) / 60.0) as u8;
                increase_date = increase_date
                    .increase_as_new(TimeSpan::hours(minute_floor as i32).into())
                    .unwrap();
                increase_date.minute = increase_date.minute + (minutes % 60) as u8;
                if increase_date.minute > 59 {
                    increase_date.minute = increase_date.minute - 60;
                }
            }
            TimeSpan::hours(hours) => {
                let hour_floor = floor(((increase_date.hour as i32 + hours) as f32) / 24.0) as u8;
                increase_date = increase_date
                    .increase_as_new(TimeSpan::days(hour_floor as i32))
                    .unwrap();
                increase_date.hour = increase_date.hour + (hours % 24) as u8;
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
                    *month_lengths.get_mut(&2).unwrap() =
                        if increase_date.isLeapYear() { 29 } else { 28 };
                    // needs to be initialized each loop because leap year changes.
                    if increase_date.day as i32 + day_counter
                        > *month_lengths.get(&(month_counter as i32)).unwrap() as i32
                    {
                        day_counter -= *month_lengths.get(&(month_counter as i32)).unwrap_or(&0);
                        month_counter += 1;
                        if month_counter == 13 {
                            month_counter = 1;
                        }
                        increase_date = increase_date.increase_as_new(TimeSpan::months(1)).unwrap();
                    } else {
                        increase_date.day = (day_counter
                            + (if initial_day == 1 {
                                1
                            } else {
                                (initial_day) as i32
                            })) as u8;
                        break;
                    }
                }
            }
            TimeSpan::months(months) => {
                increase_date.year = increase_date.year + floor(months as f32 / 12.0);
                increase_date.month = increase_date.month + (months % 12) as u8;
                if increase_date.month > 12 {
                    increase_date.month = increase_date.month - 12;
                    increase_date.year = increase_date.year + 1;
                }
            }
            TimeSpan::years(years) => {
                increase_date.year += years;
                increase_date.day = if !increase_date.isLeapYear()
                    && increase_date.month == 2
                    && increase_date.day == 29
                {
                    28
                } else {
                    increase_date.day
                };
            }
        }
        Ok(increase_date)
    }
    /// Increases given DateTime using conversion to ordinal for days. Much better at larger increases than other, but much poorer at smaller increases.*Difference is only for TimeSpan::days
    pub fn increase_ordinally(&self, length: TimeSpan) -> Result<DateTime, &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        let mut increase_date = self.clone();
        match length {
            TimeSpan::days(days) => {
                let temp_date = increase_date.clone().to_OrdinalDate().unwrap().increase_by_days(days).unwrap().to_Date().unwrap();
                increase_date = DateTime::from(increase_date.second, increase_date.minute, increase_date.hour, temp_date.day, temp_date.month, temp_date.year).unwrap();
            },
            TimeSpan::months(months) => {
                increase_date.year = increase_date.year + floor(months as f32 / 12.0);
                increase_date.month = increase_date.month + (months % 12) as u8;
                if increase_date.month > 12 {
                    increase_date.month = increase_date.month - 12;
                    increase_date.year = increase_date.year + 1;
                }
            },
            TimeSpan::years(years) => {
                increase_date.year += years;
                increase_date.day = if !increase_date.isLeapYear()
                    && increase_date.month == 2
                    && increase_date.day == 29
                {
                    28
                } else {
                    increase_date.day
                };
            },
            TimeSpan::seconds(seconds) => {
                let second_floor =
                    floor(((increase_date.second as i32 + seconds) as f32) / 60.0) as u8;
                increase_date = increase_date
                    .increase_as_new(TimeSpan::minutes(second_floor as i32).into())
                    .unwrap();
                increase_date.second = increase_date.second + (seconds % 60) as u8;
                if increase_date.second > 59 {
                    increase_date.second = increase_date.second - 60;
                }
            }
            TimeSpan::minutes(minutes) => {
                let minute_floor =
                    floor(((increase_date.minute as i32 + minutes) as f32) / 60.0) as u8;
                increase_date = increase_date
                    .increase_as_new(TimeSpan::hours(minute_floor as i32).into())
                    .unwrap();
                increase_date.minute = increase_date.minute + (minutes % 60) as u8;
                if increase_date.minute > 59 {
                    increase_date.minute = increase_date.minute - 60;
                }
            }
            TimeSpan::hours(hours) => {
                let hour_floor = floor(((increase_date.hour as i32 + hours) as f32) / 24.0) as u8;
                increase_date = increase_date
                    .increase_as_new(TimeSpan::days(hour_floor as i32))
                    .unwrap();
                increase_date.hour = increase_date.hour + (hours % 24) as u8;
                if increase_date.hour > 23 {
                    increase_date.hour = increase_date.hour - 24;
                }
            }
        }
        Ok(increase_date)
    }
    /// Decreases given DateTime using conversion to ordinal for days. Much better at larger increases than other, but much poorer at smaller increases.*Difference is only for TimeSpan::days. For now, there is no option for a gregorian decrease method
    pub fn decrease_ordinally(&self, length: TimeSpan) -> Result<DateTime, &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        let mut decrease_date = self.clone();
        match length {
            TimeSpan::days(days) => {
                let temp_date = decrease_date.to_OrdinalDate().unwrap().decrease_by_days(days).unwrap().to_Date().unwrap();
                decrease_date = DateTime::from(decrease_date.second, decrease_date.minute, decrease_date.hour, temp_date.day, temp_date.month, temp_date.year).unwrap();
            }
            TimeSpan::months(months) => {
                decrease_date.year = decrease_date.year - floor(months as f32 / 12.0);
                decrease_date.month = decrease_date.month - (months % 12) as u8;
                if decrease_date.month <= 0 {
                    decrease_date.month = decrease_date.month + 12;
                    decrease_date.year = decrease_date.year - 1;
                }
            }
            TimeSpan::years(years) => {
                decrease_date.year -= years;
                decrease_date.day = if !decrease_date.isLeapYear()
                    && decrease_date.month == 2
                    && decrease_date.day == 29
                {
                    28
                } else {
                    decrease_date.day
                };
            }
            _ => {
                return Err("Invalid TimeSpan specifier, make sure that you are using a valid TimeSpan for Date");
            }
        }
        Ok(decrease_date)
    }
}
impl OrdinalDate {
    /// Decreases the given OrdinalDate by the days specified(unfinished)
    pub fn decrease_by_days(&self, days: i32) -> Result<OrdinalDate, &'static str> {
        if !self.is_valid() {
            return Err("Invalid OrdinalDate");
        }
        let mut day_counter: i32 = self.day as i32 - days;
        let mut year = self.year;
        while day_counter <= 0 {
            year -= 1;
            day_counter += if isLeapYear(year) { 366 } else { 365 };
        }
        Ok(OrdinalDate {
            day: day_counter as u16,
            year: year,
        })
    }
    /// Increases the given OrdinalDate by the days specified
    pub fn increase_by_days(&self, days: i32) -> Result<OrdinalDate, &'static str> {
        if !self.is_valid() {
            return Err("Invalid OrdinalDate");
        }
        let mut day_counter: i32 = self.day as i32 + days;
        let mut year = self.year;
        while day_counter >= 0 {
            day_counter -= if isLeapYear(year) { 366 } else { 365 };
            year += 1;
        }
        year -= 1;
        day_counter += if isLeapYear(year) { 366 } else { 365 };
        Ok(OrdinalDate {
            day: day_counter as u16,
            year: year,
        })
    }
    /// Creates a new instance of OrdinalDate with all fields set to 1
    pub fn new() -> OrdinalDate {
        OrdinalDate { day: 1, year: 1 }
    }
    /// Creates a new instance of OrdinalDate with parameters given
    pub fn from(day: u16, year: i32) -> Result<OrdinalDate, &'static str> {
        if day > 366 {
            return Err("Day is out of range");
        }
        Ok(OrdinalDate {
            day: day,
            year: year,
        })
    }
    /// Creates a Date instance from an ordinal Date
    pub fn to_Date(&self) -> Result<Date, &'static str> {
        if (isLeapYear(self.year) && self.day > 366)
            || (!isLeapYear(self.year) && self.day > 365)
        {
            return Err("Day is greater than number of days in given year");
        }
        let mut month = 1;
        let mut day_counter: i16 = self.day as i16;
        while day_counter > 0 {
            day_counter -= days_in_month(month, self.year) as i16;
            month += 1;
        }
        day_counter += days_in_month(month - 1, self.year) as i16;
        month -= 1;
        Ok(Date {
            day: day_counter as u8,
            month: month as u8,
            year: self.year,
        })
    }
    /// Creates a DateTime instance from an ordinal Date starting new fields at 00:00:00
    pub fn to_DateTime(&self) -> Result<DateTime, &'static str> {
        if (isLeapYear(self.year) && self.day > 366)
            || (!isLeapYear(self.year) && self.day > 365)
        {
            return Err("Day is greater than number of days in given year");
        }
        let mut month = 1;
        let mut day_counter: i16 = self.day as i16;
        while day_counter > 0 {
            day_counter -= days_in_month(month, self.year) as i16;
            month += 1;
        }
        day_counter += days_in_month(month - 1, self.year) as i16;
        month -= 1;
        Ok(DateTime {
            second: 0,
            minute: 0,
            hour: 0,
            day: day_counter as u8,
            month: month as u8,
            year: self.year,
        })
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
impl Clone for OrdinalDate {
    fn clone(&self) -> OrdinalDate {
        OrdinalDate {
            day: self.day,
            year: self.year,
        }
    }
}

