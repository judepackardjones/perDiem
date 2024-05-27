use crate::types::*;

pub fn floor(num: f32) -> i32 {
    (num - (num % 1.0)) as i32
}
// not useful right now but I want to keep
// pub fn compare_dyn_any_values(a: &dyn Any, b: &dyn Any) -> Result<bool, &'static str> {
//     if let Some(a_value) = a.downcast_ref::<i8>() {
//         if let Some(b_value) = b.downcast_ref::<i8>() {
//             return Ok(a_value == b_value);
//         }
//     } else if let Some(a_value) = a.downcast_ref::<i16>() {
//         if let Some(b_value) = b.downcast_ref::<i16>() {
//             return Ok(a_value == b_value);
//         }
//     } 
//     else if let Some(a_value) = a.downcast_ref::<i32>() {
//         if let Some(b_value) = b.downcast_ref::<i32>() {
//             return Ok(a_value == b_value);
//         }
//     } 
//     Err("Values not of same type") // Values are not of the same type or the downcast failed.
    
// }

impl dateStr for &str {
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

pub fn get_pos(first: i32, second: i32) -> i32 {
    if (first - second) >= 0 {
        first - second
    } else {
        (first - second) * -1
    }
}

pub fn compare_nums(first: i32, second: i32) -> two_nums {
    match first > second {
        true => two_nums::larger,
        false => {
            if first < second {
                two_nums::smaller
            } else {
                two_nums::equal
            }
        }
    }
}
