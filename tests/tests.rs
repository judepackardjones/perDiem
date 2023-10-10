
#[cfg(test)]
mod tests {
    use perDiem::types::{Date, DateTime, datekind};


    #[test]
    fn date_to_weekday() {
        assert_eq!(Date{ day: 7, month: 10,  year: 1500}.weekday().unwrap(), "Sunday");
    }
    #[test]
    fn datetime_to_weekday() {
        assert_eq!(DateTime{ second: 5, minute: 5, hour: 5, day: 7, month: 10,  year: 1500}.weekday().unwrap(), "Sunday");
    }
    #[test]
    fn is_leap_year() {
    assert_eq!(Date{ day: 7, month: 10,  year: 1900}.isLeapYear(), false);
    }
    #[test]
    fn same_fields() {
        assert_eq!(Date {day: 15, month: 4, year: 1943}.sharesDay(&Date {day: 15, month: 5, year: 1900}), true);
        assert_eq!(Date {day: 15, month: 5, year: 1900}.sharesYear(&Date {day: 15, month: 5, year: 1900}), true);
    }
}
