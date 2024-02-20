
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
        assert_eq!(DateTime{ second: 2, minute: 5, hour: 5, day: 5, month: 10,  year: 1500}.difference(&DateTime{ second: 5, minute: 5, hour: 5, day: 7, month: 10,  year: 1490}), TimeDifference {seconds: 3, minutes: 0, hours: 0, days: 2, months: 0, years: 10});
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
        fn ordinal() {
            assert_eq!(Date::from_ordinal(OrdinalDate::from(1, 2000).unwrap()).unwrap(), Date{day: 1, month: 1, year: 2000});
            assert_eq!(Date::from_ordinal(OrdinalDate::from(32, 2000).unwrap()).unwrap(), Date{day: 1, month: 2, year: 2000});
            assert_eq!(Date::from(1, 2, 2000).unwrap().to_ordinal().unwrap(), OrdinalDate::from(32, 2000).unwrap());
        }
        #[test]
        fn increases_date() {
            assert_eq!(Date {day: 1, month: 1, year: 2003}.increase_as_new(TimeSpan::years(2)).unwrap(), Date {day: 1, month: 1, year: 2005});
            assert_eq!(Date {day: 1, month: 1, year: 2003}.increase_as_new(TimeSpan::months(2)).unwrap(), Date {day: 1, month:3, year: 2003});
            assert_eq!(Date {day: 1, month: 1, year: 2003}.increase_as_new(TimeSpan::months(11)).unwrap(), Date {day: 1, month:12, year: 2003});
            assert_eq!(Date {day: 1, month: 1, year: 2003}.increase_as_new(TimeSpan::months(12)).unwrap(), Date {day: 1, month:1, year: 2004});
            assert_eq!(Date {day: 1, month: 12, year: 2003}.increase_as_new(TimeSpan::months(12)).unwrap(), Date {day: 1, month:12, year: 2004});
            assert_eq!(Date {day: 1, month: 8, year: 2000}.increase_as_new(TimeSpan::months(26)).unwrap(), Date {day: 1, month: 10, year: 2002});
            assert_eq!(Date {day: 1, month: 1, year: 2000}.increase_as_new(TimeSpan::days(1)).unwrap(), Date {day: 2, month: 1, year: 2000});
            assert_eq!(Date {day: 1, month: 1, year: 2000}.increase_as_new(TimeSpan::days(500)).unwrap(), Date {day: 15, month: 5, year: 2001});
            assert_eq!(Date {day: 6, month: 9, year: 1987}.increase_as_new(TimeSpan::days(2000)).unwrap(), Date {day: 26, month: 2, year: 1993});
            assert_eq!(Date {day: 28, month: 2, year: 1987}.increase_as_new(TimeSpan::days(2000)).unwrap(), Date {day: 20, month: 8, year: 1992});
            assert_eq!(Date {day: 28, month: 2, year: 1980}.increase_as_new(TimeSpan::days(1)).unwrap(), Date {day: 29, month: 2, year: 1980});
        }
        #[test]
        fn increase_tests_date(){
            let mut date = Date { day: 1, month: 1, year: 2000};
            assert_eq!(Date {day: 1, month: 1, year: 2003}.increase_as_new(TimeSpan::years(0)).unwrap(), Date {day: 1, month: 1, year: 2003});
            date.increase(TimeSpan::years(1)).unwrap();
            assert_eq!(date, Date {day: 1, month: 1, year: 2001});
            date.increase(TimeSpan::months(14)).unwrap();
            assert_eq!(date, Date {day: 1, month: 3, year: 2002});
            date.increase(TimeSpan::days(200)).unwrap();
            assert_eq!(date, Date {day: 17, month: 9, year: 2002});
        }
        #[test]
        fn increases_as_new_date_time() {
            let mut example_datetime = DateTime { second: 4, minute: 20, hour: 14, day: 19, month: 3, year: 2010};
            assert_eq!(example_datetime.increase_as_new(TimeSpan::years(2)).unwrap(), DateTime { second: 4, minute: 20, hour: 14, day: 19, month: 3, year: 2012});
            example_datetime = DateTime { second: 4, minute: 20, hour: 12, day: 19, month: 3, year: 2010};
            assert_eq!(example_datetime.increase_as_new(TimeSpan::hours(48)).unwrap(), DateTime { second: 4, minute: 20, hour: 12, day: 21, month: 3, year: 2010});
            example_datetime = DateTime { second: 4, minute: 20, hour: 23, day: 19, month: 3, year: 2010};
            assert_eq!(example_datetime.increase_as_new(TimeSpan::minutes(40)).unwrap(), DateTime { second: 4, minute: 0, hour: 0, day: 20, month: 3, year: 2010 });
            example_datetime = DateTime { second: 20, minute: 59, hour: 23, day: 31, month: 12, year: 2010};
            assert_eq!(example_datetime.increase_as_new(TimeSpan::seconds(40)).unwrap(), DateTime { second: 0, minute: 0, hour: 0, day: 1, month: 1, year: 2011 });
        }
        #[test]
        fn increases_date_time() {
            let mut datetime = DateTime { second: 0, minute: 0, hour: 0, day: 1, month: 1, year: 2000};
            datetime.increase(TimeSpan::years(2)).unwrap();
            assert_eq!(datetime, DateTime { second: 0, minute: 0, hour: 0, day: 1, month: 1, year: 2002});
            datetime.increase(TimeSpan::months(14)).unwrap();
            assert_eq!(datetime, DateTime { second: 0, minute: 0, hour: 0, day: 1, month: 3, year: 2003});
            datetime.increase(TimeSpan::days(200)).unwrap();
            assert_eq!(datetime, DateTime { second: 0, minute: 0, hour: 0, day: 17, month: 9, year: 2003});
            datetime.increase(TimeSpan::hours(49)).unwrap();
            assert_eq!(datetime, DateTime { second: 0, minute: 0, hour: 1, day: 19, month: 9, year: 2003});
            datetime.increase(TimeSpan::minutes(40)).unwrap();
            assert_eq!(datetime, DateTime { second: 0, minute: 40, hour: 1, day: 19, month: 9, year: 2003});
            datetime.increase(TimeSpan::minutes(20)).unwrap();
            assert_eq!(datetime, DateTime { second: 0, minute: 0, hour: 2, day: 19, month: 9, year: 2003});
            datetime.increase(TimeSpan::seconds(40)).unwrap();
            assert_eq!(datetime, DateTime { second: 40, minute: 0, hour: 2, day: 19, month: 9, year: 2003});
            datetime.increase(TimeSpan::seconds(20)).unwrap();
            assert_eq!(datetime, DateTime { second: 0, minute: 1, hour: 2, day: 19, month: 9, year: 2003});
        }
        #[test]
        fn decrease_as_new_date() {
            let date = Date {day: 3, month: 1, year: 2000};
            assert_eq!(date.decrease_as_new(TimeSpan::years(2)).unwrap(), Date {day: 3, month: 1, year: 1998});
            assert_eq!(date.decrease_as_new(TimeSpan::months(13)).unwrap(), Date {day: 3, month: 12, year: 1998});
            assert_eq!(date.decrease_as_new(TimeSpan::days(2)).unwrap(), Date {day: 1, month: 1, year: 2000});
            assert_eq!(date.decrease_as_new(TimeSpan::days(14)).unwrap(), Date{day: 20, month: 1, year: 2000});
            assert_eq!(date.decrease_as_new(TimeSpan::days(32)).unwrap(), Date{day: 3, month: 1, year: 2000})
        }
    }
}

