#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use chrono::prelude::*;
/*This is a public crate for various date related utilities
This crate relies on the chrono crate. 
Completed functions and methods:
use method weekday to return the day of a week from a given Date struct 
use method weekday_as_int to return the day of the week as an index from 0-6 starting on Sunday. *both of these use Zellers congruence for calculations
use current_date to get the current date as Date
TODO:
Increment date method
Decrease date method
date parsing
formatting Date as String
measuring time between dates
is before
is after
has same day, month, year
which are same of dates
*/





/*
Important details about some functions:
increase and decrease date, when increasing by month, will convert the day of month to the next month: October 2nd to November 2nd, November 2nd to December 2nd.
If you try to increase month and the first month has more days then the second, it will default to the last day of that month. January 31st - Febuary 28th/29th, Febuary 28th/29th - March 28th/29th




*/
pub struct Date { // the basic struct for dates
    day: i32,
    month: i32, 
    year: i32,
}
pub struct TimeDifference {// will be used to show the amount of distance of each between dates. It will be the difference across all I.E. : Oct 7 and nov 9 2023: 2 days, 1 month
    days: i32,
    months: i32,
    years: i32,
}
pub enum TimeSpan { // will be used for increase and decrease methods 
    day(i32),
    month(i32),
    year(i32),
}


impl Date {

    pub fn isLeapYear(&self) -> bool {
        if self.year % 4 == 0 && self.year % 100 != 0{
            return true;
        } 
        if self.year % 400 == 0 {
            return true;
        }
        false
    }
    pub fn current_date() -> Date {
        let local: DateTime<Local> = Local::now();
        Date {
            day: local.day() as i32,
            month: local.month() as i32,
            year: local.year() as i32,

        }
    } 
    pub fn weekday(&self) -> Result<String, std::io::Error> {
        let weekdays: Vec<&str> = vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];                                                                 
        return Ok(weekdays[self.weekday_as_int().expect("Error converting date to week number") as usize].to_string());
    }
    pub fn weekday_as_int(&self) -> Result<i32, std::io::Error> {
        let last_two_digits_year: i32 = self.year.to_string().as_str().chars().take(2).map(|x| x.to_string()).collect::<String>().parse().unwrap();
        let first_two_digits_year: i32 = self.year as i32 % 100;      
    return Ok((self.day + 
        ((13*(if self.month == 1 || self.month == 2{
        self.month + 10
        } else {
        self.month - 2
        }
        ) - 1 ) / 5 ) + first_two_digits_year + (first_two_digits_year / 4) + (last_two_digits_year / 4) - 2*last_two_digits_year) % 7);
    }

}

pub fn SameDayOfMonth(date1: Date, date2: Date) -> bool {
    if date1.day == date2.day {
        return true;
    }
    false
}



#[macro_export]
macro_rules! increase_date {
    ($($time_span:expr, TimeSpan), *) => {
        todo!();
    }
}
#[macro_export]
macro_rules! allShare { // macro checks if every element has the same of one or more fields and returns them
    ($($time_span:expr, TimeSpan), *) => {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_to_weekday() {
        assert_eq!(Date{ day: 7, month: 10,  year: 1500}.weekday().unwrap(), "Sunday");
    }
    #[test]
    fn isLeapYear() {
    assert_eq!(Date{ day: 7, month: 10,  year: 1900}.isLeapYear(), false);
    }
}
