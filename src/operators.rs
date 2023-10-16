use crate::types::*;
#[macro_export]
macro_rules! increase_date {
    ($($time_span:expr, TimeSpan), *) => {
        todo!();
    };
}
#[macro_export]
macro_rules! decrease_date {
    ($($time_span:expr, TimeSpan), *) => {
        todo!();
    };
}

macro_rules! impl_operators_fns {
    ($struct:ident) => {
        impl crate::types::datekindOperators for $struct {
            fn last_two_digits_year(&self) -> String {
                self.year
                    .to_string()
                    .as_str()
                    .chars()
                    .rev()
                    .take(2)
                    .map(|x| x.to_string())
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>()
            }
        }
    };
}

impl_operators_fns!(Date);
impl_operators_fns!(DateTime);

impl Date {
    pub fn to_DateTime(&self) -> DateTime {
        DateTime {
            second: 0,
            minute: 0,
            hour: 0,
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}
impl DateTime {
    pub fn to_Date(&self) -> Date {
        Date {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}