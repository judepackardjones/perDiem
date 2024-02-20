use crate::types::*;
use crate::utils::{compare_dyn_any_values, compare_nums};
use chrono::prelude::Local;
use chrono::Datelike;
use chrono::{DateTime as chronoDateTime, Timelike};
use struct_iterable::Iterable;
// Implments these functions for Date and DateTime
macro_rules! impl_eval_fns {
    ($struct:ident) => {
        impl crate::types::datekindEvals for $struct {
            /// Method returns bool if Date or DateTime is start of month.
            fn isStartOfMonth(&self) -> bool {
                if self.day == 1 {
                    return true;
                }
                false
            }
            /// Method returns bool if Date or DateTime is end of month.
            fn isEndOfMonth(&self) -> bool {
                let mut month_lengths: std::collections::HashMap<i32, i32> = std::collections::HashMap::from([
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
                if self.day == *month_lengths.get(&(self.month as i32)).unwrap() as u8 {
                    return true;
                }
                false
            }
            /// Method returns bool if Date or DateTime is start of year.
            fn isStartOfYear(&self) -> bool {
                if self.month == 1 && self.day == 1 {
                    return true;
                }
                false
            }
            /// Method returns bool if Date or DateTime is end of year.
            fn isEndOfYear(&self) -> bool {
                if self.month == 12 && self.day == 31 {
                    return true;
                }
                false
            }
            
            /// Method pass Date or DateTime and returns true if Date or DateTime's year field is a leap year
            fn isLeapYear(&self) -> bool {
                (self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0
            }
            /// Method returns the day of the week as a String of the Date or DateTime passed to it.
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
                    "Saturday",
                ];
                return Ok(weekdays[self
                    .weekday_as_int()
                    .expect("Error converting date to week number")
                    as usize]
                    .to_string());
            }
            /// Method returns the day of the week as a i8 with 0 being Sunday
            fn weekday_as_int(&self) -> Result<u8, &str> {
                if !self.is_valid() {
                    return Err("Invalid Date or DateTime");
                }
                let first_two_digits_year: i32 = self.year as i32 % 100;
                let mut num: u8 = ((self.day
                    + ((13
                        * (if self.month == 1 || self.month == 2 {
                            self.month + 10
                        } else {
                            self.month - 2
                        })
                        - 1)
                        / 5)
                    + first_two_digits_year as u8
                    + (first_two_digits_year as u8 / 4)
                    + (self
                        .last_two_digits_year()
                        .parse::<u8>()
                        .expect("Failed to unwrap last two digits to i8")
                        / 4)
                    - 2 * self
                        .last_two_digits_year()
                        .parse::<u8>()
                        .expect("Failed to unwrap last two digits to i8"))
                    % 7
                    + 1);
                if num == 7 {
                    num = 0;
                }
                return Ok(num);
            }
            /// returns true if both params share the same day
            fn sharesDay(&self, date2: &$struct) -> bool {
                if self.day == date2.day {
                    return true;
                }
                false
            }
            /// returns true if both params share the same year
            fn sharesYear(&self, date2: &$struct) -> bool {
                if self.year == date2.year {
                    return true;
                }
                false
            }
            /// returns true if both params share the same month
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
    pub fn is_valid(&self) -> bool {
        let month_lengths: std::collections::HashMap<i32, i32> = std::collections::HashMap::from([
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
        if self.day > 0
            && self.day <= *month_lengths.get(&(self.month as i32)).unwrap() as u8
            && self.month > 0
            && self.month < 13
            && self.year % 1 == 0
        {
            true
        } else {
            println!("{:?}", self);
            false
        }
    }
    /// Returns a Vector of &str shared by each Date in Vector params (Returns the same as allShare, just different implementation of it.)
    pub fn allShareEL(vec: Vec<Date>) -> Vec<&'static str> {
        let mut terms: Vec<&'static str> = vec!["day", "month", "year"];
        let mut shared_terms: Vec<&'static str> = vec![];
        let base = Date {
            day: vec.get(0).expect("Vector has length 0").day,
            month: vec.get(0).expect("Interior Vector modified by external").month,
            year: vec.get(0).expect("Interior Vector modified by external").year,
        };
        for date in vec {
            for ((field_name, field_value), (_, base_value)) in date.iter().zip(base.iter()) {
                if !compare_dyn_any_values(field_value, base_value).unwrap() {
                    if let Some(index) = terms.iter().position(|&x| x == field_name) {
                        shared_terms.push(terms[index]);
                        terms.remove(terms.iter().position(|&x| x == field_name).unwrap());
                    }
                }
            }
        }
        terms
    }
    /// Returns a Vector of the shared fields in each Date from Vector
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
    pub fn days_in_month(&self) -> i8 { 
        match self.month {
          1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
          2 if self.isLeapYear() => 29,
          2 => 28,
          4 | 6 | 9 | 11 => 30,
          _ => panic!("Months should be represented as an enum")
        }
      }
    /// same function of sharesDay, sharesMonth, sharesYear, but adds comparison field as a param.
    pub fn DateShares(&self, datetime2: &Date, compare_type: &str) -> Result<bool, &str> {
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
    /// takes self and Date fields and returns true if self is after the second, date, and false if not.
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
    /// The reverse of is_after
    pub fn is_before(&self, date: Date) -> bool {
        !self.is_after(date)
    }
}

impl DateTime {
    /// Checks if a DateTime is a valid day
    pub fn is_valid(&self) -> bool {
        if (Date {
            day: self.day,
            month: self.month,
            year: self.year,
        })
        .is_valid()
            && self.second >= 0
            && self.second < 60
            && self.minute >= 0
            && self.minute < 60
            && self.hour >= 0
            && self.hour < 24
        {
            true
        } else {
            false
        }
    }
    /// Takes a Vector of DateTimes and returns all field values they share as a vector of &str. (Same function  of allShare just different implementation)
    pub fn allShareEL(vec: Vec<DateTime>) -> Vec<&'static str> {
        let mut terms: Vec<&'static str> = vec!["second", "minute", "hour", "day", "month", "year"];
        let mut shared_terms: Vec<&'static str> = vec![];
        let base = DateTime {
            second: vec.get(0).expect("Date Vector has no terms").second,
            minute: vec.get(0).unwrap().minute,
            hour: vec.get(0).unwrap().hour,
            day: vec.get(0).unwrap().day,
            month: vec.get(0).unwrap().month,
            year: vec.get(0).unwrap().year,
        };
        for date in vec {
            for ((field_name, field_value), (_, base_value)) in date.iter().zip(base.iter()) {
                if !compare_dyn_any_values(field_value, base_value).unwrap() {
                    if let Some(index) = terms.iter().position(|&x| x == field_name) {
                        shared_terms.push(terms[index]);
                        terms.remove(terms.iter().position(|&x| x == field_name).unwrap());
                    }
                }
            }
        }
        terms
    }
    /// Returns the # of days in the month of the Date/DateTime(Credit to TDark on Rust Discord)
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
    pub fn sharesSecond(&self, datetime2: DateTime) -> bool {
        if self.second == datetime2.second {
            return true;
        }
        false
    }
    /// Returns true if two DateTimes passed share the same minute value
    pub fn sharesMinute(&self, datetime2: DateTime) -> bool {
        if self.minute == datetime2.minute {
            return true;
        }
        false
    }
    /// Returns true if two DateTimes passed share the same hour value
    pub fn sharesHour(&self, datetime2: DateTime) -> bool {
        if self.hour == datetime2.hour {
            return true;
        }
        false
    }
    /// Returns true if two DateTimes passed share the same compare_type passed
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
    pub fn is_valid(&self) -> bool {
        if isLeapYear(self.year) && self.day > 366 || !isLeapYear(self.year) && self.day > 365 {
            return false;
        }
        true
    }

}
/// Method pass Date or DateTime and returns true if Date or DateTime's year field is a leap year
pub fn isLeapYear(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
pub fn days_in_month(month: u8, year: i32) -> i8 { 
    match month {
      1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
      2 if isLeapYear(year) => 29,
      2 => 28,
      4 | 6 | 9 | 11 => 30,
      _ => panic!("Months should be represented as an enum")
    }
  }
impl_eval_fns!(Date);
impl_eval_fns!(DateTime);
