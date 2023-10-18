use undup::undup_chars;
use crate::types::*;

impl x for String {
    fn as_Date(&self, format: &str) -> Date {
        let rtn_string: String = self.clone();
        let parts: Vec<&str> = rtn_string.split(|x| x == '/' || x == ',' || x == '-' || x == '\\' || x == '.').collect();
        let format_order = tap::Tap::tap_mut(undup_chars(format, vec!['d', 'm', 'y']), |s| s.retain(|c| !r#"/,-\."#.contains(c)));
        let mut day: i32 = 1;
        let mut month: i8 = 1;
        let mut year: i16 = 1;
        let mut counter_format: usize = 0;
        for i in format_order.chars() {
            match i {
                'd' => {day = parts.get(counter_format).expect("Iterator likely out of bounds").parse::<i32>().expect("Failed to parse day field to i8."); 
                counter_format += 1;},
                'm' => {month = parts.get(counter_format).expect("Iterator likely out of bounds").parse::<i8>().expect("Failed to parse month field to i8."); 
                counter_format += 1;},
                'y' => {year = parts.get(counter_format).expect("Iterator likely out of bounds").parse::<i16>().expect("Failed to parse year field to i16."); 
                counter_format += 1;},
                _ => {},
            }
        }
        Date {
            day: day as i8,
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
