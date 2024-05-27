use crate::evals::{days_in_month, isLeapYearSimple};
use crate::{types::*, utils::floor, utils::get_pos};




impl Date {
    /// Returns the last two digits of the given year
    /// 
    /// # Example
    /// 
    ///~~~~
    /// 
    /// use perDiem::types::Date;
    /// 
    /// assert_eq!(Date::from(1, 1, 2021).unwrap().last_two_digits_year(), "21".to_string());
    pub fn last_two_digits_year(&self) -> String {
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
    /// Converts given Date to OrdinalDate
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{Date, OrdinalDate};
    /// 
    /// assert_eq!(Date::from(1, 1, 2021).unwrap().to_OrdinalDate().unwrap(), OrdinalDate::from(1, 2021).unwrap());
    /// assert_eq!(Date::from(25, 5, 2021).unwrap().to_OrdinalDate().unwrap(), OrdinalDate::from(145, 2021).unwrap());
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
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::Date;
    /// 
    /// let mut date = Date::from(20, 11, 2021).unwrap();
    /// date.start_of_year();
    /// assert_eq!(date, Date::from(1, 1, 2021).unwrap());
    pub fn start_of_year(&mut self) {
        *self = Date {
            day: 1,
            month: 1,
            year: self.year,
        };
    }
    /// Changes Date to be start of current month
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::Date;
    /// 
    /// let mut date = Date::from(20, 11, 2021).unwrap();
    /// date.start_of_month();
    /// assert_eq!(date, Date::from(1, 11, 2021).unwrap());
    pub fn start_of_month(&mut self) {
        *self = Date {
            day: 1,
            month: self.month,
            year: self.year,
        };
    }
    /// Changes Date to be end of current year
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::Date;
    /// 
    /// let mut date = Date::from(20, 11, 2021).unwrap();
    /// date.end_of_year();
    /// assert_eq!(date, Date::from(31, 12, 2021).unwrap());
    pub fn end_of_year(&mut self) {
        *self = Date {
            day: 31,
            month: 12,
            year: self.year,
        };
    }
    /// Changes Date to be end of current month
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::Date; 
    /// 
    /// let mut date = Date::from(20, 11, 2021).unwrap();
    /// date.end_of_month();
    /// assert_eq!(date, Date::from(30, 11, 2021).unwrap());
    pub fn end_of_month(&mut self) {
        *self = Date {
            day: self.clone().days_in_month() as u8,
            month: self.month,
            year: self.year,
        }
    }
    /// Increases the receiver-in-place Date by the TimeSpan specified and returns a Result.
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{Date, TimeSpan};
    /// 
    /// let mut date = Date::from(20, 11, 2021).unwrap();
    /// date.increase(TimeSpan::days(5)).unwrap();
    /// assert_eq!(date, Date::from(25, 11, 2021).unwrap());
    pub fn increase(&mut self, length: TimeSpan) -> Result<(), &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        *self = self.increase_as_new(length.clone()).unwrap();
        Ok(())
    }
    /// Increases the receiver-in-place Date by the TimeSpan specified and returns a result(same result as increase method, just better at larger increases)
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{Date, TimeSpan};
    /// 
    /// let mut date = Date::from(20, 11, 2021).unwrap();
    /// date.increase_ordinally(TimeSpan::days(5));
    /// assert_eq!(date, Date::from(25, 11, 2021).unwrap());
    pub fn increase_ordinally(&mut self, length: TimeSpan) -> Result<(), &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        *self = self.increase_ordinally_as_new(length.clone()).unwrap();
        Ok(())
    }
    /// Decreases the receiver-in-place Date by the TimeSpan specified and returns a Result.
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{Date, TimeSpan};
    /// 
    /// let mut date = Date::from(20, 11, 2021).unwrap();
    /// date.decrease_ordinally(TimeSpan::days(5));
    /// assert_eq!(date, Date::from(15, 11, 2021).unwrap());
    pub fn decrease_ordinally(&mut self, length: TimeSpan) -> Result<(), &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        *self = self.decrease_ordinally_as_new(length.clone()).unwrap();
        Ok(())
    }
    /// Returns the difference between two Dates as a TimeDifference with seconds, minutes, and hours set to 0
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{Date, TimeDifference};
    /// 
    /// let date1 = Date::from(20, 11, 2021).unwrap();
    /// let date2 = Date::from(25, 5, 2024).unwrap();
    /// assert_eq!(date1.difference(&date2), TimeDifference {seconds: 0, minutes: 0, hours: 0, days: 5, months: 6, years: 3});
    pub fn difference(&self, date: &Date) -> TimeDifference {
        TimeDifference {
            seconds: 0,
            minutes: 0,
            hours: 0,
            days: get_pos(self.day.into(), date.day.into()),
            months: get_pos(self.month.into(), date.month.into()),
            years: get_pos(self.year.into(), date.year.into()),
        }
    }
    /// Creates an instance of Date at the start of BCE
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::Date;
    /// 
    /// assert_eq!(Date::new(), Date::from(1, 1, 0).unwrap());
    pub fn new() -> Date {
        Date {
            day: 1,
            month: 1,
            year: 0,
        }
    }
    /// Creates a instance of Date with fields provided
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::Date;
    /// 
    /// assert_eq!(Date::from(1, 1, 2021).unwrap(), Date { day: 1, month: 1, year: 2021});
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
    /// Converts Date to DateTime and sets second, minute, and hour to 0.
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{Date, DateTime};
    /// 
    /// assert_eq!(Date::from(1, 1, 2021).unwrap().to_DateTime(), DateTime::from(0, 0, 0, 1, 1, 2021).unwrap());
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
    /// Decreases given Date using conversion to ordinal for days. Much better at larger increases than other, but much poorer at smaller increases.*Difference is only for TimeSpan::days
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{Date, TimeSpan};
    /// 
    /// assert_eq!(Date::from(20, 11, 2021).unwrap().decrease_ordinally_as_new(TimeSpan::days(5)).unwrap(), Date::from(15, 11, 2021).unwrap());
    /// assert_eq!(Date::from(20, 11, 2021).unwrap().decrease_ordinally_as_new(TimeSpan::months(5)).unwrap(), Date::from(20, 6, 2021).unwrap());
    pub fn decrease_ordinally_as_new(&self, length: TimeSpan) -> Result<Date, &'static str> {
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
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{Date, TimeSpan};
    /// 
    /// assert_eq!(Date::from(20, 11, 2021).unwrap().increase_ordinally_as_new(TimeSpan::days(5)).unwrap(), Date::from(25, 11, 2021).unwrap());
    /// assert_eq!(Date::from(20, 11, 2021).unwrap().increase_ordinally_as_new(TimeSpan::months(5)).unwrap(), Date::from(20, 4, 2022).unwrap());
    pub fn increase_ordinally_as_new(&self, length: TimeSpan) -> Result<Date, &'static str> {
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
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{Date, TimeSpan};
    /// 
    /// assert_eq!(Date::from(20, 11, 2021).unwrap().increase_as_new(TimeSpan::days(5)).unwrap(), Date::from(25, 11, 2021).unwrap());
    /// assert_eq!(Date::from(20, 11, 2021).unwrap().increase_as_new(TimeSpan::months(5)).unwrap(), Date::from(20, 4, 2022).unwrap());
    pub fn increase_as_new(&self, length: TimeSpan) -> Result<Date, &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        let mut increase_date = self.clone();
        match length {
            TimeSpan::days(days) => {
                let initial_day = increase_date.day;
                let mut day_counter = days;
                let mut month_counter: i32 = increase_date.month as i32;
                loop {
                    // needs to be initialized each loop because leap year changes.
                    if increase_date.day as i32 + day_counter
                        > increase_date.days_in_month() as i32
                    {
                        day_counter -= increase_date.days_in_month() as i32;
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
                if !increase_date.isLeapYear() && increase_date.month == 2 && increase_date.day == 29 {
                    increase_date.day = 1;
                    increase_date.month = 3;
                } 
                
            }
            _ => {
                return Err("Invalid TimeSpan specifier, make sure that you are using a valid TimeSpan for the Date's increase method!");
            }
        }
        let final_date: Date = Date {
            day: increase_date.day,
            month: increase_date.month,
            year: increase_date.year,
        };
        Ok(final_date)
    }
    /// Decreases Date by given TimeSpan parameter. (Unfinished, use decrease_ordinally_as_new)
    pub fn decrease_as_new(&self, _length: TimeSpan) -> Result<Date, &'static str> { // remove length and underscores when retrying gregorian decrease
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        return Err("Decrease for Date is not yet implemented, use decrease_ordinally_as_new");
        let length = TimeSpan::days(2);
        let mut decrease_date = self.clone();
        match length {
            TimeSpan::days(_days) => {
                
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
    /// Returns the last two digits of the given year
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::DateTime;
    /// 
    /// assert_eq!(DateTime::from(0, 0, 0, 1, 1, 2021).unwrap().last_two_digits_year(), "21".to_string());
    pub fn last_two_digits_year(&self) -> String {
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
    /// Decreases the receiver-in-place DateTime by the TimeSpan specified and returns a Result. *There is currently no gregorian implementation of decrease because I'm lazy, write it for me if you want I'll add.
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{TimeSpan, DateTime};
    /// 
    /// let mut datetime = DateTime::from( 0, 0, 0, 20, 11, 2021).unwrap();
    /// datetime.decrease_ordinally(TimeSpan::days(5));
    /// assert_eq!(datetime, DateTime::from(0, 0, 0, 15, 11, 2021).unwrap());
    /// let mut datetime = DateTime::from(0, 0, 0, 20, 11, 2021).unwrap();
    /// datetime.decrease_ordinally(TimeSpan::months(5));
    /// assert_eq!(datetime, DateTime::from(0, 0, 0, 20, 6, 2021).unwrap());
    /// let mut datetime = DateTime::from(0, 0, 0, 20, 11, 2021).unwrap();
    /// datetime.decrease_ordinally(TimeSpan::minutes(5));
    /// assert_eq!(datetime, DateTime::from(0, 55, 23, 19, 11, 2021).unwrap());
    pub fn decrease_ordinally(&mut self, length: TimeSpan) -> Result<(), &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        *self = self.decrease_ordinally_as_new(length.clone()).unwrap();
        Ok(())
    }
    /// Decreases the receiver-in-place DateTime by the TimeSpan specified and returns a result
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{TimeSpan, DateTime};
    /// 
    /// let mut datetime = DateTime::from( 0, 0, 0, 20, 11, 2021).unwrap();
    /// datetime.increase_ordinally(TimeSpan::days(5));
    /// assert_eq!(datetime, DateTime::from(0, 0, 0, 25, 11, 2021).unwrap());
    /// let mut datetime = DateTime::from(0, 0, 0, 20, 11, 2021).unwrap();
    /// datetime.increase_ordinally(TimeSpan::minutes(5));
    /// assert_eq!(datetime, DateTime::from(0, 5, 0, 20, 11, 2021).unwrap());
    pub fn increase_ordinally(&mut self, length: TimeSpan) -> Result<(), &'static str> {
        if !self.is_valid() {
            return Err("Invalid DateTime");
        }
        *self = self.increase_ordinally_as_new(length.clone()).unwrap();
        Ok(())
    }
    /// Converts given DateTime to OrdinalDate
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{DateTime, OrdinalDate};
    /// 
    /// assert_eq!(DateTime::from(0, 0, 0, 1, 1, 2021).unwrap().to_OrdinalDate().unwrap(), OrdinalDate::from(1, 2021).unwrap());
    /// assert_eq!(DateTime::from(0, 0, 0, 25, 5, 2021).unwrap().to_OrdinalDate().unwrap(), OrdinalDate::from(145, 2021).unwrap());
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
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::DateTime;
    /// 
    /// let mut datetime = DateTime::from(0, 0, 0, 20, 11, 2021).unwrap();
    /// datetime.start_of_year();
    /// assert_eq!(datetime, DateTime::from(0, 0, 0, 1, 1, 2021).unwrap());
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
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::DateTime;
    /// 
    /// let mut datetime = DateTime::from(0, 0, 0, 20, 11, 2021).unwrap();
    /// datetime.start_of_month();
    /// assert_eq!(datetime, DateTime::from(0, 0, 0, 1, 11, 2021).unwrap());
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
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::DateTime;
    /// 
    /// let mut datetime = DateTime::from(0, 0, 0, 20, 11, 2021).unwrap();
    /// datetime.end_of_year();
    /// assert_eq!(datetime, DateTime::from(59, 59, 23, 31, 12, 2021).unwrap());
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
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::DateTime;
    /// 
    /// let mut datetime = DateTime::from(0, 0, 0, 20, 3, 2021).unwrap();
    /// datetime.end_of_month();
    /// assert_eq!(datetime, DateTime::from(59, 59, 23, 31, 3, 2021).unwrap());
    pub fn end_of_month(&mut self) {
        
        *self = DateTime {
            day: self.clone().days_in_month() as u8,
            month: self.month,
            year: self.year,
            second: 59,
            minute: 59,
            hour: 23,
        }
    }
    /// Mutates the receiver DateTime by the TimeSpan specified and returns a Result.
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{TimeSpan, DateTime};
    /// let mut datetime = DateTime::from(0, 0, 0, 20, 11, 2021).unwrap();
    /// datetime.increase(TimeSpan::days(5));
    /// assert_eq!(datetime, DateTime::from(0, 0, 0, 25, 11, 2021).unwrap());
    /// let mut datetime = DateTime::from(0, 0, 0, 20, 11, 2021).unwrap();
    /// datetime.increase(TimeSpan::months(5));
    /// assert_eq!(datetime, DateTime::from(0, 0, 0, 20, 4, 2022).unwrap());
    /// let mut datetime = DateTime::from(0, 0, 0, 20, 11, 2021).unwrap();
    /// datetime.increase(TimeSpan::minutes(5));
    /// assert_eq!(datetime, DateTime::from(0, 5, 0, 20, 11, 2021).unwrap());
    pub fn increase(&mut self, length: TimeSpan) -> Result<(), &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        *self = self.increase_as_new(length.clone()).unwrap();
        Ok(())
    }
    /// Creates new instance of DateTime started at the zeroth second of the BCE
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::DateTime;
    /// 
    /// assert_eq!(DateTime::new(), DateTime::from(0, 0, 0, 1, 1, 0).unwrap());
    pub fn new() -> DateTime {
        DateTime {
            second: 0,
            minute: 0,
            hour: 0,
            day: 1,
            month: 1,
            year: 0,
        }
    }
    /// Creates a new instance of DateTime with parameters given
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::DateTime;
    /// 
    /// assert_eq!(DateTime::from(45, 2, 0, 1, 1, 2000).unwrap(), DateTime { second: 45, minute: 2, hour: 0, day: 1, month: 1, year: 2000});
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
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{DateTime, TimeDifference};
    /// 
    /// let datetime1 = DateTime::from(5, 4, 0, 20, 11, 2021).unwrap();
    /// let datetime2 = DateTime::from(7, 7, 0, 25, 5, 2024).unwrap();
    /// assert_eq!(datetime1.difference(&datetime2), TimeDifference{seconds: 2, minutes: 3, hours: 0, days: 5, months: 6, years: 3});
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
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{DateTime, Date};
    /// 
    /// assert_eq!(DateTime::from(0, 0, 0, 20, 11, 2021).unwrap().to_Date(), Date::from(20, 11, 2021).unwrap());
    pub fn to_Date(&self) -> Date {
        Date {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
    /// Increases the given DateTime by the TimeSpan specified
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{DateTime, TimeSpan};
    /// 
    /// assert_eq!(DateTime::from(0, 0, 0, 20, 11, 2021).unwrap().increase_as_new(TimeSpan::days(5)).unwrap(), DateTime::from(0, 0, 0, 25, 11, 2021).unwrap());
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
                let mut day_counter = days;
                let mut month_counter: i32 = increase_date.month as i32;
                loop {
                    if increase_date.day as i32 + day_counter
                        > increase_date.days_in_month() as i32
                    {
                        day_counter -= increase_date.days_in_month() as i32;
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
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{DateTime, TimeSpan};
    /// 
    /// assert_eq!(DateTime::from(0, 0, 0, 20, 11, 2021).unwrap().increase_ordinally_as_new(TimeSpan::days(5)).unwrap(), DateTime::from(0, 0, 0, 25, 11, 2021).unwrap());
    /// assert_eq!(DateTime::from(0, 0, 0, 20, 11, 2021).unwrap().increase_ordinally_as_new(TimeSpan::months(5)).unwrap(), DateTime::from(0, 0, 0, 20, 4, 2022).unwrap());
    /// assert_eq!(DateTime::from(0, 0, 0, 20, 11, 2021).unwrap().increase_ordinally_as_new(TimeSpan::minutes(5)).unwrap(), DateTime::from(0, 5, 0, 20, 11, 2021).unwrap());
    pub fn increase_ordinally_as_new(&self, length: TimeSpan) -> Result<DateTime, &'static str> {
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
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{DateTime, TimeSpan};
    ///
    /// assert_eq!(DateTime::from(0, 0, 0, 20, 11, 2021).unwrap().decrease_ordinally_as_new(TimeSpan::days(5)).unwrap(), DateTime::from(0, 0, 0, 15, 11, 2021).unwrap());
    /// assert_eq!(DateTime::from(0, 0, 0, 20, 11, 2021).unwrap().decrease_ordinally_as_new(TimeSpan::months(5)).unwrap(), DateTime::from(0, 0, 0, 20, 6, 2021).unwrap());
    /// assert_eq!(DateTime::from(0, 0, 0, 20, 11, 2021).unwrap().decrease_ordinally_as_new(TimeSpan::minutes(5)).unwrap(), DateTime::from(0, 55, 23, 19, 11, 2021).unwrap());
    /// assert_eq!(DateTime::from(0, 0, 0, 15, 10, 2022).unwrap().decrease_ordinally_as_new(TimeSpan::minutes(60)).unwrap(), DateTime::from(0, 0, 0, 14, 10, 2022).unwrap());
    pub fn decrease_ordinally_as_new(&self, length: TimeSpan) -> Result<DateTime, &'static str> {
        if !self.is_valid() {
            return Err("Invalid Date");
        }
        let mut decrease_date = self.clone();
        match length {
            TimeSpan::minutes(minutes) => {
                decrease_date.decrease_ordinally(TimeSpan::hours(floor(minutes as f32 / 60.0))).unwrap();
                let temp_minute = decrease_date.minute as i8 - (minutes % 60) as i8;
                println!("Temp minute {}", temp_minute);
                if temp_minute <= 0 {
                    decrease_date.minute = (temp_minute + 60) as u8;
                    decrease_date.decrease_ordinally(TimeSpan::hours(1)).unwrap();
                } else {
                    decrease_date.minute = temp_minute as u8;
                }
            }
            TimeSpan::hours(hours) => {
                decrease_date.decrease_ordinally(TimeSpan::days(floor(hours as f32 / 24.0))).unwrap();
                let temp_hour: i8 = decrease_date.hour as i8 - (hours % 24) as i8;
                if temp_hour <= 0 {
                    decrease_date.hour = (temp_hour + 24) as u8;
                    decrease_date.decrease_ordinally(TimeSpan::days(1)).unwrap();
                } else {
                    decrease_date.hour = temp_hour as u8;
                }
            }
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
    /// Decreases the given OrdinalDate by the days specified
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::OrdinalDate;
    /// 
    /// assert_eq!(OrdinalDate::from(20, 2021).unwrap().decrease_by_days(5).unwrap(), OrdinalDate::from(15, 2021).unwrap());
    /// assert_eq!(OrdinalDate::from(20, 2021).unwrap().decrease_by_days(20).unwrap(), OrdinalDate::from(366, 2020).unwrap());
    pub fn decrease_by_days(&self, days: i32) -> Result<OrdinalDate, &'static str> {
        if !self.is_valid() {
            return Err("Invalid OrdinalDate");
        }
        let mut day_counter: i32 = self.day as i32 - days;
        let mut year = self.year;
        while day_counter <= 0 {
            year -= 1;
            day_counter += if isLeapYearSimple(year) { 366 } else { 365 };
        }
        Ok(OrdinalDate {
            day: day_counter as u16,
            year: year,
        })
    }
    /// Increases the given OrdinalDate by the days specified
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::OrdinalDate;
    /// 
    /// assert_eq!(OrdinalDate::from(20, 2021).unwrap().increase_by_days(5).unwrap(), OrdinalDate::from(25, 2021).unwrap());
    /// assert_eq!(OrdinalDate::from(20, 2021).unwrap().increase_by_days(20).unwrap(), OrdinalDate::from(40, 2021).unwrap());
    pub fn increase_by_days(&self, days: i32) -> Result<OrdinalDate, &'static str> {
        if !self.is_valid() {
            return Err("Invalid OrdinalDate");
        }
        let mut day_counter: i32 = self.day as i32 + days;
        let mut year = self.year;
        while day_counter >= 0 {
            day_counter -= if isLeapYearSimple(year) { 366 } else { 365 };
            year += 1;
        }
        year -= 1;
        day_counter += if isLeapYearSimple(year) { 366 } else { 365 };
        Ok(OrdinalDate {
            day: day_counter as u16,
            year: year,
        })
    }
    /// Creates a new instance of OrdinalDate with at the start of BCE
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::OrdinalDate;
    /// assert_eq!(OrdinalDate::new(), OrdinalDate::from(1, 0).unwrap());
    pub fn new() -> OrdinalDate {
        OrdinalDate { day: 1, year: 0 }
    }
    /// Creates a new instance of OrdinalDate with parameters given
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::OrdinalDate;
    /// 
    /// assert_eq!(OrdinalDate::from(20, 2021).unwrap(), OrdinalDate{day: 20, year: 2021});
    pub fn from(day: u16, year: i32) -> Result<OrdinalDate, &'static str> {
        if day > 365 && !isLeapYearSimple(year) || day > 366 && isLeapYearSimple(year){
            return Err("Day is out of range for year");
        }
        Ok(OrdinalDate {
            day: day,
            year: year,
        })
    }
    /// Creates a Date instance from an OrdinalDate
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{OrdinalDate, Date};
    /// 
    /// assert_eq!(OrdinalDate::from(20, 2021).unwrap().to_Date().unwrap(), Date::from(20, 1, 2021).unwrap());
    pub fn to_Date(&self) -> Result<Date, &'static str> {
        if (isLeapYearSimple(self.year) && self.day > 366)
            || (!isLeapYearSimple(self.year) && self.day > 365)
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
    /// 
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::{OrdinalDate, DateTime};
    /// 
    /// assert_eq!(OrdinalDate::from(20, 2021).unwrap().to_DateTime().unwrap(), DateTime::from(0, 0, 0, 20, 1, 2021).unwrap());
    pub fn to_DateTime(&self) -> Result<DateTime, &'static str> {
        if (isLeapYearSimple(self.year) && self.day > 366)
            || (!isLeapYearSimple(self.year) && self.day > 365)
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

