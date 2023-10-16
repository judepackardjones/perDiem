#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_macros)]
/*This is a public crate for various date related utilities
This crate relies on the chrono crate.
Completed functions and methods:
IsLeapYear method checks if given Date or DateTime was a leap year.
weekday method returns day of week of given Date or DateTime
weekday_as_int returns day of week of given Date or DateTime as int from 0-6, starting Sunday.
sharesday, sharesmonth, and sharesyear methods return bool of if self and given Date/DateTime have the same of given field.
sharessecond, sharesminute, shareshour methods return bool of if self and given DateTime have the same of given field.
Dateshares and DateTimeshares are the same as the methods above but you specify the field you want to compare. 
last_two_digits_year returns convert Date -> DateTime or DateTime -> Date. Date -> DateTime starts with fields second: 0, minute: 0, hour: 0,.
to_string method converts Date to given format as String. 
snapshot_date and snapshot_datetime gets the current date/datetime and converts it into Date or DateTime, respectively. 
to_DateTime and to_Date take the other as a parameter and return the date as the other.
TODO:
Increment date macro
Decrease date macro
date parsing
measuring time between dates
is before
is after
which fields are same of list of dates(macro)
*/

/*
Important details about some functions:
increase and decrease date, when increasing by month, will convert the day of month to the next month: October 2nd to November 2nd, November 2nd to December 2nd.
If you try to increase month and the first month has more days then the second, it will default to the last day of that month. January 31st - Febuary 28th/29th, Febuary 28th/29th - March 28th/29th




*/
pub mod evals;
pub mod operators;
pub mod types;
pub mod textmanagement {
    pub mod formatting;
    pub mod parsing;
}
