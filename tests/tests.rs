
#[cfg(test)]
mod tests {

    mod weekdays {
        use perDiem::types::{Date, DateTime, datekindEvals};

    #[test]
    fn date_to_weekday_tags() {
        assert_eq!(Date{ day: 7, month: 10,  year: 1500}.weekday().unwrap(), "Sunday");
    }
    #[test]
    fn datetime_to_weekday_evals_tags() {
        assert_eq!(DateTime{ second: 5, minute: 5, hour: 5, day: 7, month: 10,  year: 1500}.weekday().unwrap(), "Sunday");
    }
    }
    mod dateconditions {
        use perDiem::types::{Date, datekindEvals};

    #[test]
    fn is_leap_year() {
    assert_eq!(Date{ day: 7, month: 10,  year: 1900}.isLeapYear(), false);
    }
    }
    mod comparisons {
        use perDiem::types::{Date, datekindEvals};

    #[test]
    fn same_fields_shared() {
        assert_eq!(Date {day: 15, month: 4, year: 1943}.sharesDay(&Date {day: 15, month: 5, year: 1900}), true);
        assert_eq!(Date {day: 14, month: 5, year: 1943}.sharesMonth(&Date {day: 15, month: 5, year: 1940}), true);
        assert_eq!(Date {day: 24, month: 1, year: 1980}.sharesYear(&Date {day: 15, month: 2, year: 1980}), true);
    }
    }
}
