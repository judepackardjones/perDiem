use crate::types::*;
use chrono::prelude::Local;
use chrono::Datelike;
use chrono::{DateTime as chronoDateTime, Timelike};
use struct_iterable::Iterable;
use crate::utils::compare_dyn_any_values;
#[macro_export]
macro_rules! impl_eval_fns {
    ($struct:ident) => {
        impl crate::types::datekindEvals for $struct {
            fn isLeapYear(&self) -> bool {
                if (self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0 {
                    return true;
                }
                false
            }
            fn weekday(&self) -> Result<String, std::io::Error> {
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
            fn weekday_as_int(&self) -> Result<i8, std::io::Error> {
                let first_two_digits_year: i32 = self.year as i32 % 100;
                let mut num: i8 = ((self.day
                    + ((13
                        * (if self.month == 1 || self.month == 2 {
                            self.month + 10
                        } else {
                            self.month - 2
                        })
                        - 1)
                        / 5)
                    + first_two_digits_year as i8
                    + (first_two_digits_year as i8 / 4)
                    + (self
                        .last_two_digits_year()
                        .parse::<i8>()
                        .expect("Failed to unwrap last two digits to i8")
                        / 4)
                    - 2 * self
                        .last_two_digits_year()
                        .parse::<i8>()
                        .expect("Failed to unwrap last two digits to i8"))
                    % 7
                    + 1);
                if num == 7 {
                    num = 0;
                }
                return Ok(num);
            }
            fn sharesDay(&self, date2: &$struct) -> bool {
                if self.day == date2.day {
                    return true;
                }
                false
            }
            fn sharesYear(&self, date2: &$struct) -> bool {
                if self.year == date2.year {
                    return true;
                }
                false
            }
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
    pub fn allShareEL(vec: Vec<Date>) -> Vec<&'static str> {
        let mut terms: Vec<&'static str> = vec!["day", "month", "year"];
        let mut shared_terms: Vec<&'static str> = vec![];
        let base = Date {
            day: vec.get(0).expect("Vec has length 0").day,
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
    pub fn allShare(vec: Vec<Date>) -> Vec<&'static str> {
        let mut shares_terms: Vec<&'static str> = vec!["day", "month", "year"];
        let (day, month, year) = (vec.get(0).expect("Date Vector has no terms").day, vec.get(0).unwrap().month, vec.get(0).unwrap().year);
        for date in vec {
            if date.day != day {
                if let Some(index) = shares_terms.iter().position(|&x| x == "day") {
                shares_terms.remove(index);
                }
            }
            if date.month != month{
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
    pub fn snapshot_date() -> crate::types::Date {
        let local: chronoDateTime<Local> = Local::now();
        crate::types::Date {
            day: local.day() as i8,
            month: local.month() as i8,
            year: local.year() as i16,
        }
    }
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
    pub fn is_after(&self, date: Date) -> bool {
        if compare_nums(self.year, date.year) == two_nums::larger {
            true
        } else if compare_nums(self.year, date.year) == two_nums::smaller {
            false
        } else if compare_nums(self.month as i16, date.month as i16) == two_nums::larger {
            true
        } else if compare_nums(self.month as i16, date.month as i16) == two_nums::smaller {
            false
        } else if compare_nums(self.day as i16, date.day as i16) == two_nums::larger {
            true
        } else if compare_nums(self.day as i16, date.day as i16) == two_nums::smaller {
            false
        } else {
            false
        }
    }
    pub fn is_before(&self, date: Date) -> bool {
        !self.is_after(date)
    }
}
fn compare_nums(first: i16, second: i16) -> two_nums {
    match first > second {
        true => two_nums::larger,
        false => {
            if first < second {
                two_nums::smaller
            } else {
                two_nums::equal
            }
        }
    }
}

impl DateTime {
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
pub fn allShare(vec: Vec<DateTime>) -> Vec<&'static str> {
    let mut shares_terms: Vec<&'static str> = vec!["second","minute","hour","day", "month", "year"];
    let (second, minute, hour, day, month, year) = (vec.get(0).expect("Date Vector has no terms").second, vec.get(0).unwrap().minute, vec.get(0).unwrap().hour, vec.get(0).unwrap().day, vec.get(0).unwrap().month, vec.get(0).unwrap().year);
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
        if date.month != month{
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
    pub fn snapshot_datetime() -> crate::types::DateTime {
        let local: chronoDateTime<Local> = Local::now();
        crate::types::DateTime {
            second: local.second() as i8,
            minute: local.minute() as i8,
            hour: local.hour() as i8,
            day: local.day() as i8,
            month: local.month() as i8,
            year: local.year() as i16,
        }
    }
    pub fn sharesSecond(&self, datetime2: DateTime) -> bool {
        if self.second == datetime2.second {
            return true;
        }
        false
    }
    pub fn sharesMinute(&self, datetime2: DateTime) -> bool {
        if self.minute == datetime2.minute {
            return true;
        }
        false
    }
    pub fn sharesHour(&self, datetime2: DateTime) -> bool {
        if self.hour == datetime2.hour {
            return true;
        }
        false
    }
    pub fn DateTimeShares(&self, compare_type: &str, datetime2: &Self) -> Result<bool, &str> {
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

impl_eval_fns!(Date);
impl_eval_fns!(DateTime);
