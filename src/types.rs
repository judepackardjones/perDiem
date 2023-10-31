
use struct_iterable::Iterable;
#[derive(Debug, PartialEq, Eq, Iterable)]
/// The basic Date struct
pub struct Date {
    /// Contains the day number of the Date instance
    pub day: i8,
    /// Contains the month number of the Date instance
    pub month: i8,
    /// Contains the year number of the Date instance
    pub year: i32,
}

#[derive(Debug, PartialEq, Eq, Iterable)]
/// The basic DateTime struct
pub struct DateTime {
    pub second: i8,
    pub minute: i8,
    pub hour: i8,
    pub day: i8,
    pub month: i8,
    pub year: i32,
}
#[derive(Debug, PartialEq, Eq, Iterable)]
/// Used as a measure of distance between dates
pub struct TimeDifference {
    // will be used to show the amount of distance of each between dates. It will be the difference across all I.E. : Oct 7 and nov 9 2023: 2 days, 1 month
    /// Contains the second difference
    pub seconds: i32,
    /// Contains the minute difference
    pub minutes: i32,
    /// Contains the hour difference
    pub hours: i32,
    /// Contains the day difference 
    pub days: i32,
    /// Contains the month difference 
    pub months: i32,
    /// Contains the year difference 
    pub years: i32,
}
/// Unimplemented enum for increase and decrease methods
pub enum TimeSpan {
    // will be used for increase and decrease methods
    seconds(i32),
    minutes(i32),
    hours(i32),
    days(i32),
    months(i32),
    years(i32),
}
/// Eval methods that are impl by macro for both Date and DateTime
pub trait datekindEvals {
    /// Returns if an instance of Date or DateTime's year struct is a leap year
    fn isLeapYear(&self) -> bool;
    /// Returns the day of the week a certain Date or DateTime is as a String. 
    fn weekday(&self) -> Result<String, &str>;
    /// Returns the day of the week as an i8. 0 = Sunday. 
    fn weekday_as_int(&self) -> Result<i8, &str>;
    /// Returns bool from if Date or DateTime share day field.
    fn sharesDay(&self, date2: &Self) -> bool;
    /// Returns bool from if Date or DateTime share month field.
    fn sharesMonth(&self, date2: &Self) -> bool;
    /// Returns bool from if Date or DateTime share year field.
    fn sharesYear(&self, date2: &Self) -> bool;
}
/// Operator methods that are impl by macro for both Date and Dateime
pub trait datekindOperators {
    fn last_two_digits_year(&self) -> String;
}
/// Trait used for creating String methods.
pub trait x {
    // used for creating String methods
    fn as_Date(&self, format: &str) -> Date;
}
/// Trait used for creating &str methods.
pub trait y {
    // used for creating &str methods
    fn with_separators(&self, separator: &char) -> String;
}
#[derive(PartialEq)]
pub enum two_nums {
    larger,
    smaller,
    equal,
}   
