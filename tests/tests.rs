
#[cfg(test)]
mod tests {
    mod weekdays {
        use perDiem::types::{Date, DateTime, datekindEvals};

    #[test]
    fn date_to_weekday() {
        assert_eq!(Date{ day: 9, month: 10,  year: 1500}.weekday().unwrap(), "Tuesday");
    }
    #[test]
    fn datetime_to_weekday_evals() {
        assert_eq!(DateTime{ second: 5, minute: 5, hour: 5, day: 7, month: 10,  year: 1500}.weekday().unwrap(), "Sunday");
        assert_eq!(DateTime{ second: 5, minute: 5, hour: 5, day: 8, month: 10,  year: 1500}.weekday().unwrap(), "Monday");
        assert_eq!(DateTime{ second: 5, minute: 5, hour: 5, day: 9, month: 10,  year: 1500}.weekday().unwrap(), "Tuesday");
        assert_eq!(DateTime{ second: 5, minute: 5, hour: 5, day: 10, month: 10,  year: 1500}.weekday().unwrap(), "Wednesday");
        assert_eq!(DateTime{ second: 5, minute: 5, hour: 5, day: 11, month: 10,  year: 1500}.weekday().unwrap(), "Thursday");
        assert_eq!(DateTime{ second: 5, minute: 5, hour: 5, day: 12, month: 10,  year: 1500}.weekday().unwrap(), "Friday");
        assert_eq!(DateTime{ second: 5, minute: 5, hour: 5, day: 13, month: 10,  year: 1500}.weekday().unwrap(), "Saturday");
        assert_eq!(DateTime{ second: 5, minute: 5, hour: 5, day: 14, month: 10,  year: 1500}.weekday().unwrap(), "Sunday");
    }
    }
    mod dateconditions {
        use perDiem::types::{Date, datekindEvals};

    #[test]
    fn is_leap_year() {
    assert_eq!(Date{ day: 7, month: 10,  year: 1900}.isLeapYear(), false);
    }
    mod day_comparisons {
    use perDiem::types::*;
    mod time_differences {
        use perDiem::types::*;
        #[test]
        fn time_differences() {
        assert_eq!(DateTime{ second: 2, minute: 5, hour: 5, day: 5, month: 10,  year: 1500}.difference(DateTime{ second: 5, minute: 5, hour: 5, day: 7, month: 10,  year: 1490}), TimeDifference {seconds: 3, minutes: 0, hours: 0, days: 2, months: 0, years: 10});
        }
    }
    #[test]
    fn is_after_test() {
        assert!(Date{ day: 6, month: 10, year: 1900}.is_after(Date {day: 5, month: 10, year: 1900}));
        assert_ne!(Date{ day: 6, month: 10, year: 1900}.is_after(Date {day: 5, month: 8, year: 1600}), false);
        assert_eq!(Date{ day: 6, month: 10, year: 1900}.is_after(Date {day: 20, month: 10, year: 1900}), false);
    }
    #[test]
    fn is_before_test() {
        assert!(Date{ day: 4, month: 10, year: 1900}.is_before(Date {day: 10, month: 10, year: 1900}));
        assert_ne!(Date{ day: 3, month: 8, year: 1900}.is_before(Date {day: 5, month: 8, year: 1600}), true);
        assert_eq!(Date{ day: 30, month: 10, year: 1900}.is_before(Date {day: 20, month: 10, year: 1900}), false);
    }
}
    }
    mod comparisons {
        use perDiem::types::{Date, datekindEvals, DateTime};
    #[test] 
    fn is_valid_tests() {
        assert_eq!(Date {day: 29, month: 2, year: 2000}.is_valid(), true);
        assert_ne!(Date {day: 29, month: 2, year: 2001}.is_valid(), true);
        assert_eq!(DateTime {second: 59, minute: 59, hour: 1, day: 29, month: 3, year: 2000}.is_valid(), true);
        assert_ne!(DateTime {second: 59, minute: 59, hour: 24, day: 29, month: 3, year: 2000}.is_valid(), true);
    }
    #[test]
    fn all_shares_test() {
        let compare_vec: Vec<&str> = vec!["second", "minute", "hour", "day"];
        assert_eq!(DateTime::allShare(vec![DateTime {second:1, minute: 1, hour: 1, day: 2, month: 6, year: 2000}, DateTime {second:1, minute: 1, hour: 1, day: 2, month: 5, year: 1990}]), compare_vec);
    }
    #[test]
    fn same_fields_shared() {
        assert_eq!(Date {day: 15, month: 4, year: 1943}.sharesDay(&Date {day: 15, month: 5, year: 1900}), true);
        assert_eq!(Date {day: 14, month: 5, year: 1943}.sharesMonth(&Date {day: 15, month: 5, year: 1940}), true);
        assert_eq!(Date {day: 24, month: 1, year: 1980}.sharesYear(&Date {day: 15, month: 2, year: 1980}), true);
    }
    }
    mod texttests {

        mod parses{
            use perDiem::types::*;
            #[test]
            fn as_date_test() {
                assert_eq!(Date {day: 1, month: 2, year: 2000}, String::from("01/02/2000").as_Date("dd/mm/yyyy"));
            }
        }
        mod formatting{
            use perDiem::types::*;
            #[test]
            fn separators_insert() {
                let strr: &str = "ddmmyyyy";
            assert_eq!(strr.with_separators(&'/'), String::from("dd/mm/yyyy"));
            }
            #[test]
            fn to_string_test() {
                assert_eq!(Date {day: 1, month: 2, year: 2003}.to_string("ddmmyy", &'/').unwrap(), String::from("01/02/03"));
            }
        }
    }
    mod operators {
        use perDiem::types::*;
        #[test]
        fn last_two_digits_year_test() {
            assert_eq!(Date {day: 1, month: 1, year: 2003}.last_two_digits_year(), String::from("03"));
        }
        #[test]
        fn increases() {
            assert_eq!(Date {day: 1, month: 1, year: 2003}.increase(TimeSpan::years(2)).unwrap(), Date {day: 1, month: 1, year: 2005});
            assert_eq!(Date {day: 1, month: 1, year: 2003}.increase(TimeSpan::months(2)).unwrap(), Date {day: 1, month:3, year: 2003});
            assert_eq!(Date {day: 1, month: 1, year: 2003}.increase(TimeSpan::months(11)).unwrap(), Date {day: 1, month:12, year: 2003});
            assert_eq!(Date {day: 1, month: 1, year: 2003}.increase(TimeSpan::months(12)).unwrap(), Date {day: 1, month:1, year: 2004});
            assert_eq!(Date {day: 1, month: 12, year: 2003}.increase(TimeSpan::months(12)).unwrap(), Date {day: 1, month:12, year: 2004});
            assert_eq!(Date {day: 1, month: 8, year: 2000}.increase(TimeSpan::months(26)).unwrap(), Date {day: 1, month: 10, year: 2002});
            assert_eq!(Date {day: 1, month: 1, year: 2000}.increase(TimeSpan::days(500)).unwrap(), Date {day: 1, month: 2, year: 2000});
        }
    }
}
