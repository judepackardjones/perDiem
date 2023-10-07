#![allow(non_snake_case)]
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
//is it fixed>
*/
pub struct Date { 
    day: i32,
    month: i32, 
    year: i32,
}
impl Date {
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
        let first_two_digits_year: i32 = self.year % 100;      
    return Ok((self.day + 
        ((13*(if self.month == 1 || self.month == 2{
        self.month + 10
        } else {
        self.month - 2
        }
        ) - 1 ) / 5 ) + first_two_digits_year + (first_two_digits_year / 4) + (last_two_digits_year / 4) - 2*last_two_digits_year) % 7) 
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Date{ day: 7, month: 10,  year: 1500}.weekday().unwrap(), "Sunday");
    }
}
