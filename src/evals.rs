use crate::types::*;
use crate::utils::compare_nums;
use chrono::prelude::Local;
use chrono::Datelike;
use chrono::{DateTime as chronoDateTime, Timelike};

// Implements these functions for Date and DateTime
macro_rules! impl_eval_fns {
    ($struct:ident) => {
        impl crate::types::datekindEvals for $struct {
            /// Method returns bool if Date or DateTime is start of month.
            ///
            /// # Example
            /// 
            ///~~~~
            /// use perDiem::types::Date;
            /// use perDiem::types::datekindEvals;
            /// 
            /// assert_eq!(Date::from(1, 1, 2000).unwrap().isStartOfMonth(), true);
            /// assert_eq!(Date::from(15, 1, 2000).unwrap().isStartOfMonth(), false);
            fn isStartOfMonth(&self) -> bool {
                if self.day == 1 {
                    return true;
                }
                false
            }
            /// Method returns bool if Date or DateTime is end of month.
            ///
            /// # Example
            /// 
            ///~~~~
            /// use perDiem::types::Date; 
            /// use perDiem::types::datekindEvals;
            /// 
            /// assert_eq!(Date::from(29, 2, 2000).unwrap().isEndOfMonth(), true);
            /// assert_eq!(Date::from(31, 3, 2000).unwrap().isEndOfMonth(), true);
            /// assert_eq!(Date::from(15, 1, 2000).unwrap().isEndOfMonth(), false);
            fn isEndOfMonth(&self) -> bool {
                if self.day == self.days_in_month() as u8 {
                    return true;
                }
                false
            }
            /// Method returns bool if Date or DateTime is start of year.
            ///
            /// # Example
            /// 
            ///~~~~
            /// use perDiem::types::Date;
            /// use perDiem::types::datekindEvals;
            /// 
            /// assert_eq!(Date::from(1, 1, 2000).unwrap().isStartOfYear(), true);
            /// assert_eq!(Date::from(15, 1, 2000).unwrap().isStartOfYear(), false);
            fn isStartOfYear(&self) -> bool {
                if self.month == 1 && self.day == 1 {
                    return true;
                }
                false
            }
            /// Method returns bool if Date or DateTime is end of year.
            ///
            /// # Example
            /// 
            ///~~~~
            /// use perDiem::types::Date; 
            /// use perDiem::types::datekindEvals;
            /// 
            /// assert_eq!(Date::from(31, 12, 2000).unwrap().isEndOfYear(), true);
            /// assert_eq!(Date::from(15, 1, 2000).unwrap().isEndOfYear(), false);
            fn isEndOfYear(&self) -> bool {
                if self.month == 12 && self.day == 31 {
                    return true;
                }
                false
            }
            
            /// Method pass Date or DateTime and returns true if Date or DateTime's year field is a leap year 
            ///
            /// # Example
            /// 
            ///~~~~
            /// use perDiem::types::Date; 
            /// use perDiem::types::datekindEvals;
            /// 
            /// assert_eq!(Date::from(1, 1, 2000).unwrap().isLeapYear(), true);
            /// assert_eq!(Date::from(1, 1, 2001).unwrap().isLeapYear(), false);
            /// assert_eq!(Date::from(1, 1, 1900).unwrap().isLeapYear(), false); // Make sure you understand leap year rules
            fn isLeapYear(&self) -> bool {
                (self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0
            }
            /// Method returns the day of the week as a String of the Date or DateTime passed to it.
            ///
            /// # Example
            /// 
            ///~~~~
            /// use perDiem::types::Date; 
            /// use perDiem::types::datekindEvals;
            /// 
            /// assert_eq!(Date::from(22, 5, 2024).unwrap().weekday().unwrap(), "Wednesday");
            /// assert_eq!(Date::from(23, 5, 2024).unwrap().weekday().unwrap(), "Thursday");
            fn weekday(&self) -> Result<String, &str> {
                if !self.is_valid() {
                    return Err("Invalid Date or DateTime");
                }
                let weekdays: Vec<&str> = vec![
                    "Sunday",
                    "Monday",
                    "Tuesday",
                    "Wednesday",
                    "Thursday",
                    "Friday",
                    "Saturday"
                ];
                println!("In weekday {}", self.weekday_as_int().unwrap());
                return Ok(weekdays[self
                    .weekday_as_int()
                    .expect("Error converting date to week number")
                    as usize]
                    .to_string());
            }
            /// Method returns the day of the week as a i8 with 0 being Sunday
            /// 
            /// # Example
            /// use perDiem::evals::weekday_as_int;
            /// 
            /// println!("Out test {}", Date::from(23, 5, 2024).unwrap().weekday_as_int().unwrap());
            /// assert_eq!(Date::from(23, 5, 2024).unwrap().weekday_as_int().unwrap(), 4);
            /// assert_eq!(Date::from(24, 5, 2024).unwrap().weekday_as_int().unwrap(), 5);
            fn weekday_as_int(&self) -> Result<u8, &str> {
                if !self.is_valid() {
                    return Err("Invalid Date or DateTime");
                }
                let d: u32= self.day as u32;
                let m: u8 = if self.month < 3 { self.month + 10 } else { self.month - 2 }; // January and February are counted as months 13 and 14 of the previous year
                let Y: i32 = if m > 10 { self.year - 1 } else { self.year };
                let c: u32 = (Y as f64 / 100.0).floor() as u32;
                let y: u32 = Y as u32 - 100 * c;
                let weekday = (d + (2.6 * m as f32 - 0.2).floor() as u32 + y + (y as f32 / 4.0).floor() as u32 + (c as f32 / 4.0).floor() as u32 - 2 * c) % 7;
                Ok(weekday as u8)
                
            }
            /// Returns true if both params share the same day of month
            ///
            /// # Example
            /// 
            ///~~~~
            /// use perDiem::types::Date;
            /// use perDiem::types::datekindEvals;
            /// 
            /// assert_eq!(Date::from(1, 3, 2002).unwrap().sharesDay(&Date::from(1, 5, 2020).unwrap()), true);
            /// assert_eq!(Date::from(1, 3, 2020).unwrap().sharesDay(&Date::from(2, 3, 2020).unwrap()), false);
            fn sharesDay(&self, date2: &$struct) -> bool {
                if self.day == date2.day {
                    return true;
                }
                false
            }
            /// Returns true if both params share the same year
            ///
            /// # Example
            /// 
            ///~~~~
            /// use perDiem::types::Date;
            /// use perDiem::types::datekindEvals;
            /// 
            /// assert_eq!(Date::from(5, 3, 2002).unwrap().sharesYear(&Date::from(1, 5, 2002).unwrap()), true);
            /// assert_eq!(Date::from(1, 3, 2000).unwrap().sharesYear(&Date::from(2, 3, 2020).unwrap()), false);
            fn sharesYear(&self, date2: &$struct) -> bool {
                if self.year == date2.year {
                    return true;
                }
                false
            }
            /// Returns true if both params share the same month
            ///
            /// # Example
            /// 
            ///~~~~
            /// use perDiem::types::Date;
            /// use perDiem::types::datekindEvals;
            /// 
            /// assert_eq!(Date::from(1, 3, 2002).unwrap().sharesMonth(&Date::from(15, 3, 2020).unwrap()), true);
            /// assert_eq!(Date::from(1, 3, 2000).unwrap().sharesMonth(&Date::from(2, 4, 2020).unwrap()), false);
            fn sharesMonth(&self, date2: &$struct) -> bool {
                if self.month == date2.month {
                    return true;
                }
                false
            }
        }
    };
}
impl Date {
    /// Checks if a Date is valid
    ///
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::Date;
    /// 
    /// assert_eq!(Date { day: 1, month: 1, year: 2000}.is_valid(), true);
    /// assert_eq!(Date { day: 29, month: 2, year: 2001}.is_valid(), false);
    /// assert_eq!(Date { day: 50, month: 4, year: 2000}.is_valid(), false);
    pub fn is_valid(&self) -> bool {
        if self.day > 0
            && self.day <= self.days_in_month() as u8
            && self.month > 0
            && self.month < 13
            && self.year % 1 == 0
        {
            true
        } else {
            false
        }
    }
    // might be reimplemented later
    // pub fn allShareEL(vec: Vec<Date>) -> Vec<&'static str> {
    //     let mut terms: Vec<&'static str> = vec!["day", "month", "year"];
    //     let mut shared_terms: Vec<&'static str> = vec![];
    //     let base = Date {
    //         day: vec.get(0).expect("Vector has length 0").day,
    //         month: vec.get(0).expect("Interior Vector modified by external").month,
    //         year: vec.get(0).expect("Interior Vector modified by external").year,
    //     };
    //     for date in vec {
    //         for ((field_name, field_value), (_, base_value)) in date.iter().zip(base.iter()) {
    //             if !compare_dyn_any_values(field_value, base_value).unwrap() {
    //                 if let Some(index) = terms.iter().position(|&x| x == field_name) {
    //                     shared_terms.push(terms[index]);
    //                     terms.remove(terms.iter().position(|&x| x == field_name).unwrap());
    //                 }
    //             }
    //         }
    //     }
    //     terms
    // }
    /// Returns a Vector of the shared fields in each Date from Vector
    ///
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::Date;
    /// 
    /// let shares_vec = Date::allShare(vec![Date::from(1, 1, 2001).unwrap(), Date::from(1, 1, 2000).unwrap()]);
    /// assert_eq!(shares_vec.contains(&"day"), true);
    /// assert_eq!(shares_vec.contains(&"month"), true);
    /// assert_eq!(shares_vec.contains(&"year"), false);
    pub fn allShare(vec: Vec<Date>) -> Vec<&'static str> {
        let mut shares_terms: Vec<&'static str> = vec!["day", "month", "year"];
        let (day, month, year) = (
            vec.get(0).expect("Date Vector has no terms").day,
            vec.get(0).unwrap().month,
            vec.get(0).unwrap().year,
        );
        for date in vec {
            if date.day != day {
                if let Some(index) = shares_terms.iter().position(|&x| x == "day") {
                    shares_terms.remove(index);
                }
            }
            if date.month != month {
                if let Some(index) = shares_terms.iter().position(|&x| x == "month") {
                    shares_terms.remove(index);
                }
            }
            if date.year != year {
                if let Some(index) = shares_terms.iter().position(|&x| x == "year") {
                    shares_terms.remove(index);
                }
            }
        }
        shares_terms
    }
    /// Takes a snapshot of the current date in Local time
    pub fn snapshot_date() -> crate::types::Date {
        let local: chronoDateTime<Local> = Local::now();
        crate::types::Date {
            day: local.day() as u8,
            month: local.month() as u8,
            year: local.year() as i32,
        }
    }
    /// Returns the # of days in the month of the Date/DateTime(Credit to TDark on Rust Discord)
    ///
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::Date;
    /// 
    /// assert_eq!(Date::from(1, 1, 2000).unwrap().days_in_month(), 31);
    /// assert_eq!(Date::from(1, 2, 2000).unwrap().days_in_month(), 29);
    pub fn days_in_month(&self) -> i8 { 
        match self.month {
          1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
          2 if self.isLeapYear() => 29,
          2 => 28,
          4 | 6 | 9 | 11 => 30,
          _ => panic!("Invalid month: {}", self.month)
        }
      }
    /// same function of sharesDay, sharesMonth, sharesYear, but adds comparison field as a param.
    ///
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::Date;
    /// 
    /// assert_eq!(Date::from(1, 5, 2000).unwrap().datesShare(&Date::from(1, 10, 2001).unwrap(), "day").unwrap(), true);
    /// assert_eq!(Date::from(1, 5, 2000).unwrap().datesShare(&Date::from(1, 10, 2001).unwrap(), "month").unwrap(), false);
    pub fn datesShare(&self, datetime2: &Date, compare_type: &str) -> Result<bool, &str> {
        match compare_type {
            "day" => {
                if self.day == datetime2.day {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            "month" => {
                if self.month == datetime2.month {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            "year" => {
                if self.year == datetime2.year {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            &_ => return Err("Invalid compare type"),
        }
    }
    /// Takes self and Date fields and returns true if self is after the second, date, and false if not.
    ///
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::Date;
    /// 
    /// assert_eq!(Date::from(1, 1, 2001).unwrap().is_after(Date::from(1, 1, 2000).unwrap()), true);
    /// assert_eq!(Date::from(1, 1, 2000).unwrap().is_after(Date::from(1, 1, 2001).unwrap()), false);
    pub fn is_after(&self, date: Date) -> bool {
        if compare_nums(self.year, date.year) == two_nums::larger {
            true
        } else if compare_nums(self.year, date.year) == two_nums::smaller {
            false
        } else if compare_nums(self.month as i32, date.month as i32) == two_nums::larger {
            true
        } else if compare_nums(self.month as i32, date.month as i32) == two_nums::smaller {
            false
        } else if compare_nums(self.day as i32, date.day as i32) == two_nums::larger {
            true
        } else if compare_nums(self.day as i32, date.day as i32) == two_nums::smaller {
            false
        } else {
            false
        }
    }
    /// Takes self and Date fields and returns true if self is before the second, date, and false if not.
    ///
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::Date;
    /// 
    /// assert_eq!(Date::from(1, 1, 2000).unwrap().is_before(Date::from(1, 1, 2001).unwrap()), true);
    /// assert_eq!(Date::from(1, 1, 2001).unwrap().is_before(Date::from(1, 1, 2000).unwrap()), false);
    pub fn is_before(&self, date: Date) -> bool {
        !self.is_after(date)
    }
}

impl DateTime {
    /// Checks if a DateTime is a valid day
    ///
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::DateTime;
    /// 
    /// assert_eq!(DateTime {second: 0, minute: 0, hour: 0, day: 1, month: 1, year: 2000}.is_valid(), true);
    /// assert_eq!(DateTime {second: 0, minute: 0, hour: 0, day: 29, month: 2, year: 2000}.is_valid(), true);
    /// assert_eq!(DateTime {second: 0, minute: 0, hour: 0, day: 29, month: 2, year: 2001}.is_valid(), false);
    /// assert_eq!(DateTime {second: 0, minute: 0, hour: 0, day: 50, month: 1, year: 2000}.is_valid(), false);
    /// assert_eq!(DateTime {second: 0, minute: 0, hour: 50, day: 1, month: 1, year: 2000}.is_valid(), false);
    /// assert_eq!(DateTime {second: 65, minute: 0, hour: 0, day: 1, month: 3, year: 2000}.is_valid(), false);
    pub fn is_valid(&self) -> bool {
        if (Date {
            day: self.day,
            month: self.month,
            year: self.year,
        })
        .is_valid()
            && self.second <= 60
            && self.minute <= 60
            && self.hour <= 24
        {// comparison  of 0s useless due to type limits
            true
        } else {
            false
        }
    }
    // might be reimplemented later
    // pub fn allShareEL(vec: Vec<DateTime>) -> Vec<&'static str> {
    //     let mut terms: Vec<&'static str> = vec!["second", "minute", "hour", "day", "month", "year"];
    //     let mut shared_terms: Vec<&'static str> = vec![];
    //     let base = DateTime {
    //         second: vec.get(0).expect("Date Vector has no terms").second,
    //         minute: vec.get(0).unwrap().minute,
    //         hour: vec.get(0).unwrap().hour,
    //         day: vec.get(0).unwrap().day,
    //         month: vec.get(0).unwrap().month,
    //         year: vec.get(0).unwrap().year,
    //     };
    //     for date in vec {
    //         for ((field_name, field_value), (_, base_value)) in date.iter().zip(base.iter()) {
    //             if !compare_dyn_any_values(field_value, base_value).unwrap() {
    //                 if let Some(index) = terms.iter().position(|&x| x == field_name) {
    //                     shared_terms.push(terms[index]);
    //                     terms.remove(terms.iter().position(|&x| x == field_name).unwrap());
    //                 }
    //             }
    //         }
    //     }
    //     terms
    // }

    /// Returns the # of days in the month of the Date/DateTime(Credit to TDark on Rust Discord)
    ///
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::Date;
    /// 
    /// assert_eq!(Date::from(1, 1, 2000).unwrap().days_in_month(), 31);
    /// assert_eq!(Date::from(1, 2, 2000).unwrap().days_in_month(), 29);
    pub fn days_in_month(&self) -> i8 { 
        match self.month {
          1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
          2 if self.isLeapYear() => 29,
          2 => 28,
          4 | 6 | 9 | 11 => 30,
          _ => panic!("Months should be represented as an enum")
        }
      }
    /// Takes a Vector of DateTimes and returns a Vector of &strs of the field values they share
    ///
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::DateTime;
    /// 
    /// let shares_vec = DateTime::allShare(vec![DateTime::from(0, 5, 23, 1, 1, 2001).unwrap(), DateTime::from(0, 6, 23, 1, 1, 2000).unwrap()]);
    /// assert_eq!(shares_vec.contains(&"second"), true);
    /// assert_eq!(shares_vec.contains(&"minute"), false);
    /// assert_eq!(shares_vec.contains(&"hour"), true);
    /// assert_eq!(shares_vec.contains(&"day"), true);
    /// assert_eq!(shares_vec.contains(&"month"), true);
    /// assert_eq!(shares_vec.contains(&"year"), false);
    pub fn allShare(vec: Vec<DateTime>) -> Vec<&'static str> {
        let mut shares_terms: Vec<&'static str> =
            vec!["second", "minute", "hour", "day", "month", "year"];
        let (second, minute, hour, day, month, year) = (
            vec.get(0).expect("Date Vector has no terms").second,
            vec.get(0).unwrap().minute,
            vec.get(0).unwrap().hour,
            vec.get(0).unwrap().day,
            vec.get(0).unwrap().month,
            vec.get(0).unwrap().year,
        );
        for date in vec {
            if date.second != second {
                if let Some(index) = shares_terms.iter().position(|&x| x == "second") {
                    shares_terms.remove(index);
                }
            }
            if date.minute != minute {
                if let Some(index) = shares_terms.iter().position(|&x| x == "minute") {
                    shares_terms.remove(index);
                }
            }
            if date.hour != hour {
                if let Some(index) = shares_terms.iter().position(|&x| x == "hour") {
                    shares_terms.remove(index);
                }
            }
            if date.day != day {
                if let Some(index) = shares_terms.iter().position(|&x| x == "day") {
                    shares_terms.remove(index);
                }
            }
            if date.month != month {
                if let Some(index) = shares_terms.iter().position(|&x| x == "month") {
                    shares_terms.remove(index);
                }
            }
            if date.year != year {
                if let Some(index) = shares_terms.iter().position(|&x| x == "year") {
                    shares_terms.remove(index);
                }
            }
        }
        shares_terms
    }
    /// Takes a snapshot of the current local DateTime as a DateTime
    pub fn snapshot_datetime() -> crate::types::DateTime {
        let local: chronoDateTime<Local> = Local::now();
        crate::types::DateTime {
            second: local.second() as u8,
            minute: local.minute() as u8,
            hour: local.hour() as u8,
            day: local.day() as u8,
            month: local.month() as u8,
            year: local.year() as i32,
        }
    }
    /// Returns true if two DateTimes passed share the same second value
    ///
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::DateTime;
    /// 
    /// assert_eq!(DateTime::from(50, 0, 1, 1, 1, 2000).unwrap().sharesSecond(DateTime::from(50, 0, 1, 1, 1, 2000).unwrap()), true);
    pub fn sharesSecond(&self, datetime2: DateTime) -> bool {
        if self.second == datetime2.second {
            return true;
        }
        false
    }
    /// Returns true if two DateTimes passed share the same minute value
    ///
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::DateTime;
    /// 
    /// assert_eq!(DateTime::from(50, 20, 1, 1, 1, 2000).unwrap().sharesMinute(DateTime::from(50, 20, 1, 1, 1, 2000).unwrap()), true);
    pub fn sharesMinute(&self, datetime2: DateTime) -> bool {
        if self.minute == datetime2.minute {
            return true;
        }
        false
    }
    /// Returns true if two DateTimes passed share the same hour value
    ///
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::DateTime;
    /// 
    /// assert_eq!(DateTime::from(50, 4, 16, 1, 1, 2000).unwrap().sharesHour(DateTime::from(40, 0, 16, 1, 1, 2000).unwrap()), true);
    pub fn sharesHour(&self, datetime2: DateTime) -> bool {
        if self.hour == datetime2.hour {
            return true;
        }
        false
    }
    /// Returns true if two DateTimes passed share the same compare_type passed
    ///
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::DateTime;
    /// 
    /// assert_eq!(DateTime::from(50, 4, 16, 1, 1, 2000).unwrap().DateTimeShares(&DateTime::from(40, 0, 16, 3, 2, 2020).unwrap(), "hour").unwrap(), true);
    pub fn DateTimeShares(&self, datetime2: &Self, compare_type: &str) -> Result<bool, &str> {
        match compare_type {
            "second" => {
                if self.second == datetime2.second {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            "minute" => {
                if self.minute == datetime2.minute {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            "hour" => {
                if self.hour == datetime2.hour {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            "day" => {
                if self.day == datetime2.day {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            "month" => {
                if self.month == datetime2.month {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            "year" => {
                if self.year == datetime2.year {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            &_ => {
                return Err("Invalid compare type");
            }
        }
    }
}
impl OrdinalDate {
    /// Checks if an ordinal date is valid
    ///
    /// # Example
    /// 
    ///~~~~
    /// use perDiem::types::OrdinalDate;
    /// 
    /// assert_eq!(OrdinalDate {day: 1, year: 2000}.is_valid(), true);
    /// assert_eq!(OrdinalDate {day: 366, year: 2000}.is_valid(), true);
    /// assert_eq!(OrdinalDate {day: 367, year: 2000}.is_valid(), false);
    pub fn is_valid(&self) -> bool {
        if isLeapYearSimple(self.year) && self.day > 366 || !isLeapYearSimple(self.year) && self.day > 365 {
            return false;
        }
        true
    }

}
/// Method pass Date or DateTime and returns true if Date or DateTime's year field is a leap year
/// 
/// # Example
/// 
///~~~~
/// use perDiem::evals::isLeapYearSimple;
/// 
/// assert_eq!(isLeapYearSimple(2000), true);
/// assert_eq!(isLeapYearSimple(2001), false);
/// assert_eq!(isLeapYearSimple(1900), false); 
pub fn isLeapYearSimple(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
/// Returns the # of days in the month of the Date/DateTime(Credit to TDark on Rust Discord)
/// 
/// # Example
/// 
///~~~~
/// use perDiem::evals::days_in_month;
/// 
/// assert_eq!(days_in_month(1, 2000), 31);
/// assert_eq!(days_in_month(2, 2000), 29);
pub fn days_in_month(month: u8, year: i32) -> i8 { 
    match month {
      1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
      2 if isLeapYearSimple(year) => 29,
      2 => 28,
      4 | 6 | 9 | 11 => 30,
      _ => panic!("Months should be represented as an enum")
    }
  }
impl_eval_fns!(Date);
impl_eval_fns!(DateTime);
