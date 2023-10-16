use std::collections::HashMap;

use crate::types::*;

impl x for String {
    fn as_Date(&self, format: &str) -> Date {
        let mut parts = self.split(|x| x == '/' || x == ',' || x == '-' || x == '\\' || x == '.');
        let mut seen: HashMap<char, bool> = HashMap::new();
        let format_order = format;
        format_order.chars().collect::<Vec<char>>().retain(|&x| seen.insert(x, true).is_none());
        let mut day: i8 = 0;
        let mut month: i8 = 0;
        let mut year: i16 = 0;
        for i in format_order.chars() {
            match i {
                'd' => {day = parts.next().expect("Iterator likely out of bounds").parse::<i8>().expect("Failed to parse day field to i8.");},
                'm' => {month = parts.next().expect("Iterator likely out of bounds").parse::<i8>().expect("Failed to parse month field to i8.");},
                'y' => {year = parts.next().expect("Iterator likely out of bounds").parse::<i16>().expect("Failed to parse year field to i16.");},
                _ => {},
            }
        }
        Date {
            day: day,
            month: month,
            year: year,
        }
    }
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
