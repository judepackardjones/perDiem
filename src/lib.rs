#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_macros)]
/*This is a public crate for various date related utilities
This crate relies on the chrono crate. 
It was a deliberate choice to use i32 for all date related types. I'd be happy to discuss it, it just makes the calculations of 
Completed functions and methods:
use method weekday to return the day of a week from a given Date struct 
use method weekday_as_int to return the day of the week as an index from 0-6 starting on Sunday. *both of these use Zellers congruence for calculations
use current_date to get the current date as Date
use sharesDay, sharesMonth, sharesYear methods to check if parameter 1 has same date field as self.
TODO:
Increment date macro
Decrease date macro
date parsing
formatting Date as String
measuring time between dates
is before 
is after
which fields are same of list of dates
converstion between DateTime and Date
*/





/*
Important details about some functions:
increase and decrease date, when increasing by month, will convert the day of month to the next month: October 2nd to November 2nd, November 2nd to December 2nd.
If you try to increase month and the first month has more days then the second, it will default to the last day of that month. January 31st - Febuary 28th/29th, Febuary 28th/29th - March 28th/29th




*/
pub mod operators;
pub mod evals;
pub mod types;
pub mod textmanagement {
    pub mod formatting;
    pub mod parsing;
}

