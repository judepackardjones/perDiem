
#[cfg(test)]
mod tests {
    mod weekdays {
        use perDiem::types::*;

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
        println!("In test {}", Date::from(23, 5, 2024).unwrap().weekday_as_int().unwrap());
        assert_eq!(Date::from(23, 1, 2024).unwrap().weekday_as_int().unwrap(), 2);
        assert_eq!(Date::from(24, 5, 2024).unwrap().weekday_as_int().unwrap(), 5);
    }
    #[test]
    fn weekday_tests() {
        assert_eq!(Date::from(23, 5, 2024).unwrap().weekday_as_int().unwrap(), 4);
        assert_eq!(Date::from(24, 5, 2024).unwrap().weekday_as_int().unwrap(), 5);
    }
    }
    mod dateconditions {
        use perDiem::types::*;

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
        use perDiem::types::*;
    #[test] 
    fn is_valid_tests() {
        assert_eq!(Date {day: 29, month: 2, year: 2000}.is_valid(), true);
        assert_ne!(Date {day: 29, month: 2, year: 2001}.is_valid(), true);
        assert_eq!(DateTime {second: 59, minute: 59, hour: 1, day: 29, month: 3, year: 2000}.is_valid(), true);
        assert_ne!(DateTime {second: 59, minute: 59, hour: 23, day: 29, month: 3, year: 2000}.is_valid(), false);
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
                let str: &str = "ddmmyyyy";
            assert_eq!(str.with_separators(&'/'), String::from("dd/mm/yyyy"));
            }
            #[test]
            fn to_string_test() {
                assert_eq!(Date {day: 1, month: 2, year: 2003}.to_string("ddmmyy", &'/').unwrap(), String::from("01/02/03"));
            }
        }
    }
    mod operators {
        use perDiem::types::*;
        use std::time::Instant;
        #[test]
        fn speed_test() {
            let ordinal = Instant::now();
            let _ = Date::from(1, 1, 2000).unwrap().increase_ordinally_as_new(TimeSpan::days(20));
            let ordinal = ordinal.elapsed().as_nanos();
            let gregorian = Instant::now();
            let _ = Date::from(1, 1, 2000).unwrap().increase_as_new(TimeSpan::days(20));
            let gregorian = gregorian.elapsed().as_nanos();
            println!("Ordinal: {} nanoseconds, Gregorian: {} nanoseconds", ordinal, gregorian);
            let ordinal = Instant::now();
            let _ = Date::from(1, 1, 2000).unwrap().increase_ordinally_as_new(TimeSpan::days(2000));
            let ordinal = ordinal.elapsed().as_nanos();
            let gregorian = Instant::now();
            let _ = Date::from(1, 1, 2000).unwrap().increase_as_new(TimeSpan::days(2000));
            let gregorian = gregorian.elapsed().as_nanos();
            println!("Ordinal: {} nanoseconds, Gregorian: {} nanoseconds", ordinal, gregorian);
        }
        #[test]
        fn last_two_digits_year_test() {
            assert_eq!(Date {day: 1, month: 1, year: 2003}.last_two_digits_year(), String::from("03"));
        }
        #[test]
        fn ordinal() {
            assert_eq!(OrdinalDate::from(1, 2000).unwrap().to_Date().unwrap(), Date{day: 1, month: 1, year: 2000});
            assert_eq!(OrdinalDate::from(32, 2000).unwrap().to_Date().unwrap(), Date{day: 1, month: 2, year: 2000});
            assert_eq!(Date::from(1, 2, 2000).unwrap().to_OrdinalDate().unwrap(), OrdinalDate::from(32, 2000).unwrap());
            assert_eq!(OrdinalDate::from(1, 2000).unwrap().increase_by_days(366).unwrap(), OrdinalDate::from(1, 2001).unwrap());
            assert_eq!(OrdinalDate::from(1, 2001).unwrap().decrease_by_days(1).unwrap(), OrdinalDate::from(366, 2000).unwrap());
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
            assert_eq!(Date {day: 28, month: 2, year: 1980}.increase_ordinally_as_new(TimeSpan::days(1)).unwrap(), Date {day: 29, month: 2, year: 1980});
            assert_eq!(Date {day: 6, month: 9, year: 1987}.increase_ordinally_as_new(TimeSpan::days(2000)).unwrap(), Date {day: 26, month: 2, year: 1993});
            assert_eq!(Date {day: 26, month: 2, year: 1993}.decrease_ordinally_as_new(TimeSpan::days(2000)).unwrap(), Date {day: 6, month: 9, year: 1987});
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
        fn leap_year_increase() {
            let date = Date::from(29, 2, 2012).unwrap();
            assert_eq!(date.increase_as_new(TimeSpan::years(1)), Date::from(1, 3, 2013));
        }
        #[test]
        fn increases_as_new_date_time() {
            let mut example_datetime = DateTime { second: 4, minute: 20, hour: 14, day: 19, month: 3, year: 2010};
            assert_eq!(example_datetime.increase_as_new(TimeSpan::seconds(56)).unwrap(), DateTime { second: 0, minute: 21, hour: 14, day: 19, month: 3, year: 2010});
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
    }
}

