use std::any::Any;
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
