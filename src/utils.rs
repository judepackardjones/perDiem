use std::{any::Any, collections::HashMap};
use crate::types::*;

pub fn compare_dyn_any_values(a: &dyn Any, b: &dyn Any) -> Result<bool, &'static str> {
    if let Some(a_value) = a.downcast_ref::<i8>() {
        if let Some(b_value) = b.downcast_ref::<i8>() {
            return Ok(a_value == b_value);
        }
    } else if let Some(a_value) = a.downcast_ref::<i16>() {
        if let Some(b_value) = b.downcast_ref::<i16>() {
            return Ok(a_value == b_value);
        }
    }
    Err("Values not of same type")// Values are not of the same type or the downcast failed.
}

impl y for &str {
    fn with_separators(&self, separator: &char) -> String {
        let mut result: String = String::new();
        let mut toggle: bool = false;
        let mut index: usize = 0;
        for i in self.chars() {
            match toggle {
                false => {
                    result.push(i);
                }
                true => {
                    result.push(i);
                    if i == *self
                        .chars()
                        .collect::<Vec<char>>()
                        .get(index)
                        .expect("Failed to unwrap option")
                    {
                        result.push(*separator);
                    }
                }
            }
            if toggle == true {
                toggle = !toggle;
                continue;
            }
            index += 1;
            toggle = !toggle;
        }
        if result
            .chars()
            .nth(result.len() - 1)
            .expect("Failed to unwrap Option<char> to char")
            == '/'
        {
            result.remove(result.len() - 1);
        }
        result
    }
}

impl DateTime {
    pub fn type_id(&self) -> i8 {
        1
    }
}

impl Date {
    pub fn type_id(&self) -> i8 {
        0
    }
}


    fn check_rollover(int1: i32, int2: i32, isleapYear: bool, field: TimeSpan) -> bool {
    let month_lengths: HashMap<i32, i32> = HashMap::from([
        (1, 31),
        (2, if isleapYear { 29 } else { 28 }),
        (3, 31),
        (4, 30),
        (5, 31),
        (6, 30),
        (7, 31),
        (8, 31),
        (9, 30),
        (10, 31),
        (11, 30),
        (12, 31),
    ]);
    let rollovers: HashMap<&str, i32> = HashMap::from([
        ("seconds", 60),
        ("minutes", 60),
        ("hours", 24),
        ("months", 12),
    ]);
    match field {
        TimeSpan::seconds(secs) => todo!(),
        TimeSpan::minutes(mins) => todo!(),
        TimeSpan::hours(hours) => todo!(),
        TimeSpan::days(days) => todo!(),
        TimeSpan::months(months) => todo!(),
        TimeSpan::years(years) => todo!(),
    }



}

pub fn get_pos(first: i32, second: i32) -> i32 {
    if (first - second) >= 0 {
        first - second
    } else {
        (first - second)* - 1
    }
}