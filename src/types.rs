
use struct_iterable::Iterable;
#[derive(Debug, PartialEq, Eq, Iterable)]
pub struct Date {
    pub day: i8,
    pub month: i8,
    pub year: i16,
}

#[derive(Debug, PartialEq, Eq, Iterable)]
pub struct DateTime {
    pub second: i8,
    pub minute: i8,
    pub hour: i8,
    pub day: i8,
    pub month: i8,
    pub year: i16,
}
pub struct TimeDifference {
    // will be used to show the amount of distance of each between dates. It will be the difference across all I.E. : Oct 7 and nov 9 2023: 2 days, 1 month
    pub seconds: i32,
    pub minutes: i32,
    pub hour: i32,
    pub days: i32,
    pub months: i32,
    pub years: i32,
}
pub enum TimeSpan {
    // will be used for increase and decrease methods
    seconds(i32),
    minutes(i32),
    hours(i32),
    days(i32),
    months(i32),
    years(i32),
}
pub trait datekind {}
pub trait datekindEvals {
    fn isLeapYear(&self) -> bool;
    fn weekday(&self) -> Result<String, std::io::Error>;
    fn weekday_as_int(&self) -> Result<i8, std::io::Error>;
    fn sharesDay(&self, date2: &Self) -> bool;
    fn sharesMonth(&self, date2: &Self) -> bool;
    fn sharesYear(&self, date2: &Self) -> bool;
}
pub trait datekindOperators {
    fn last_two_digits_year(&self) -> String;
}

pub trait x {
    // used for creating String methods
    fn as_Date(&self, format: &str) -> Date;
}
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
