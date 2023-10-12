use crate::types::*;
use either::Either;
#[macro_export]
macro_rules! increase_date {
    ($($time_span:expr, TimeSpan), *) => {
        todo!();
    }
}
#[macro_export]
macro_rules! decrease_date {
    ($($time_span:expr, TimeSpan), *) => {
        todo!();
    }
}

macro_rules! impl_operators_fns {
    ($struct:ident) => {
        impl crate::types::datekindOperators for $struct {
            fn not_as_datekind(&self) -> Either<Date, DateTime> {
                todo!()
            }
            fn last_two_digits_year(&self) -> i32 {
                self.year.to_string().as_str().chars().take(2).map(|x| x.to_string()).collect::<String>().parse().unwrap()
            }
        }
    }
}





impl_operators_fns!(Date);
impl_operators_fns!(DateTime);