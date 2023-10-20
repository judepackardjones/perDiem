use crate::types::*;
use chrono::prelude::Local;
use chrono::Datelike;
use chrono::{DateTime as chronoDateTime, Timelike};

#[macro_export]
macro_rules! allShare {
    ($($date:expr), *) => {{
        use perDiem::types::*;
        let mut date_vec: Vec<Date> = Vec::new();
        let mut datetime_vec: Vec<DateTime> = Vec::new();
        let mut second: Option<i8> = None;
        let mut minute: Option<i8> = None;
        let mut hour: Option<i8> = None;
        let mut day: Option<i8> = None;
        let mut month: Option<i8> = None;
        let mut year: Option<i16> = None;
        let mut shares_terms: Vec<&str> = vec!["second", "minute", "hour", "day", "month", "year"];

        $(let date = $date;
            if date.type_id() == 0 {
                shares_terms.drain(0..2);
            }
          if day.is_none() {
              day = Some(date.day as i8);
          } else if day != Some(date.day) {
                if let Some(index) = shares_terms.iter().position(|&x| x == "day") {
                    shares_terms.remove(index);
                }
            }

          if month.is_none() {
              month = Some(date.month);
          } else if month != Some(date.month) {
                if let Some(index) = shares_terms.iter().position(|&x| x == "month") {
                    shares_terms.remove(index);
                }
            }
          if year.is_none() {
              year = Some(date.year);
          } else if year != Some(date.year) {
                if let Some(index) = shares_terms.iter().position(|&x| x == "year") {
                    shares_terms.remove(index);
                }
            }

            if date.type_id() != 0 {
                if second.is_none() {
                    second = Some(date.second as i8);
                } else if second != Some(date.second) {
                      if let Some(index) = shares_terms.iter().position(|&x| x == "second") {
                          shares_terms.remove(index);
                      }
                  }
      
                if minute.is_none() {
                    minute = Some(date.minute);
                } else if minute != Some(date.minute) {
                      if let Some(index) = shares_terms.iter().position(|&x| x == "minute") {
                          shares_terms.remove(index);
                      }
                  }
                if hour.is_none() {
                    hour = Some(date.hour);
                } else if hour != Some(date.hour) {
                      if let Some(index) = shares_terms.iter().position(|&x| x == "hour") {
                          shares_terms.remove(index);
                      }
                  }
            }
            if date.type_id() == 1 {
                date_vec.push(date);
            } else {
                datetime_vec.push(date);
            }
        )*
        shares_terms
    }};
}

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
impl crate::types::Date {
    fn snapshot_date() -> crate::types::Date {
        let local: chronoDateTime<Local> = Local::now();
        crate::types::Date {
            day: local.day() as i8,
            month: local.month() as i8,
            year: local.year() as i16,
        }
    }
    fn DateShares(&self, datetime2: &Date, compare_type: &str) -> Result<bool, &str> {
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

impl crate::types::DateTime {
    fn snapshot_datetime() -> crate::types::DateTime {
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
    fn sharesSecond(&self, datetime2: DateTime) -> bool {
        if self.second == datetime2.second {
            return true;
        }
        false
    }
    fn sharesMinute(&self, datetime2: DateTime) -> bool {
        if self.minute == datetime2.minute {
            return true;
        }
        false
    }
    fn sharesHour(&self, datetime2: DateTime) -> bool {
        if self.hour == datetime2.hour {
            return true;
        }
        false
    }
    fn DateTimeShares(&self, compare_type: &str, datetime2: &Self) -> Result<bool, &str> {
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
