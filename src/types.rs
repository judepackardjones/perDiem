use either::*;
pub struct Date { // the basic struct for dates
    pub day: i8,
    pub month: i8, 
    pub year: i16,
}

pub struct DateTime {
    pub second: i8,
    pub minute: i8,
    pub hour: i8,
    pub day: i8,
    pub month: i8, 
    pub year: i16,
}
pub struct TimeDifference {// will be used to show the amount of distance of each between dates. It will be the difference across all I.E. : Oct 7 and nov 9 2023: 2 days, 1 month
    pub days: i32,
    pub months: i32,
    pub years: i32,
}
pub enum TimeSpan { // will be used for increase and decrease methods 
    day(i32),
    month(i32),
    year(i32),
}
pub trait datekind {}
pub trait datekindEvals {
    fn isLeapYear(&self) -> bool;
    fn weekday(&self) -> Result<String, std::io::Error> ;
    fn weekday_as_int(&self) -> Result<i8, std::io::Error>;
    fn sharesDay(&self, date2: &Self) -> bool;
    fn sharesMonth(&self, date2: &Self) -> bool;
    fn sharesYear(&self, date2: &Self) -> bool;
}
pub trait datekindOperators {
    fn not_as_datekind(&self) -> Either<Date, DateTime>;
}




pub trait x { // used for creating String methods
    fn as_Date(&self) -> Date;
}
