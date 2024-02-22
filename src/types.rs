use struct_iterable::Iterable;
#[derive(Debug, PartialEq, Eq, Iterable)]
/// The basic Date struct
pub struct Date {
    /// Contains the day number of the Date instance
    pub day: u8,
    /// Contains the month number of the Date instance
    pub month: u8,
    /// Contains the year number of the Date instance
    pub year: i32,
}
#[derive(Debug, PartialEq, Eq, Iterable)]
/// The basic OrdinalDate struct, used for ordinal date formats
pub struct OrdinalDate {
    /// Contains the day number of the Date instance
    pub day: u16,
    /// Contains the year number of the Date instance
    pub year: i32,
}

#[derive(Debug, PartialEq, Eq, Iterable)]
/// The basic DateTime struct
pub struct DateTime {
    /// Contains the second number of the DateTime instance
    pub second: u8,
    /// Contains the minute number of the DateTime instance
    pub minute: u8,
    /// Contains the hour number of the DateTime instance
    pub hour: u8,
    /// Contains the day number of the DateTime instance
    pub day: u8,
    /// Contains the month number of the DateTime instance
    pub month: u8,
    /// Contains the year number of the DateTime instance
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
/// Used to hold variant time amounts primarily for increase and decrease methods
pub enum TimeSpan {
    // will be used for increase and decrease methods
    /// Represents the additive(or subtractive(is that a word?)) seconds to a Date or DateTime
    seconds(i32),
    /// Represents the variant of minutes that could be added or subtracted from a Date or DateTime
    minutes(i32),
    /// Represents the variant of hours that could be added or subtracted from a Date or DateTime
    hours(i32),
    /// Represents the variant of days that could be added or subtracted from a Date or DateTime
    days(i32),
    /// Represents the variant of months that could be added or subtracted from a Date or DateTime
    months(i32),
    /// Represents the variant of years that could be added or subtracted from a Date or DateTime
    years(i32),
}
/// Eval methods that are impl by macro for both Date and DateTime
pub trait datekindEvals {
    /// Returns if an instance of Date or DateTime's year struct is a leap year
    fn isLeapYear(&self) -> bool;
    /// Returns the day of the week a certain Date or DateTime is as a String.
    fn weekday(&self) -> Result<String, &str>;
    /// Returns the day of the week as an i8. 0 = Sunday.
    fn weekday_as_int(&self) -> Result<u8, &str>;
    /// Returns bool from if Date or DateTime share day field.
    fn sharesDay(&self, date2: &Self) -> bool;
    /// Returns bool from if Date or DateTime share month field.
    fn sharesMonth(&self, date2: &Self) -> bool;
    /// Returns bool from if Date or DateTime share year field.
    fn sharesYear(&self, date2: &Self) -> bool;
    /// Returns bool from if given Date or DateTime is December 31st
    fn isEndOfYear(&self) -> bool;
    /// Returns bool from if given Date or DateTime is January 1st
    fn isStartOfYear(&self) -> bool;
    /// Returns bool from if given Date or DateTime is end of current month
    fn isEndOfMonth(&self) -> bool;
    /// Returns bool from if given Date or DateTime is start of current month
    fn isStartOfMonth(&self) -> bool;
    
}
/// Operator methods that are impl by macro for both Date and Dateime
pub trait datekindOperators {
    /// Gets the last two digits from a year. Is used internally for Zueller's congruence for leap year calculation
    fn last_two_digits_year(&self) -> String;
}
/// Trait used for creating String methods.
pub trait x {
    // used for creating String methods
    /// Takes a String formatted like a Date, and it's format and converts it to a Date instance
    fn as_Date(&self, format: &str) -> Date;
}
/// Trait used for creating &str methods.
pub trait y {
    // used for creating &str methods
    /// Used for adding separators to Date formats
    fn with_separators(&self, separator: &char) -> String;
}

#[derive(PartialEq)]
// Internally used enum for larger, smaller, and equal uses
pub enum two_nums {
    larger,
    smaller,
    equal,
}