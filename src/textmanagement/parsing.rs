use crate::types::*;
use undup::undup_chars;

impl x for String {
    fn as_Date(&self, format: &str) -> Date {
        let rtn_string: String = self.clone();
        let parts: Vec<&str> = rtn_string.split(|x| x == '/' || x == ',' || x == '-' || x == '\\' || x == '.').collect();
        let format_order = tap::Tap::tap_mut(undup_chars(format, vec!['d', 'm', 'y']), |s| {
            s.retain(|c| !r#"/,-\."#.contains(c))
        });
        let mut day: i32 = 1; // This is a stupid bandaid patch idk why but this fixed an issue. 
        let mut month: i8 = 1;
        let mut year: i32 = 1;
        let mut counter_format: usize = 0;
        for i in format_order.chars() {
            match i {
                'd' => {
                    day = parts
                        .get(counter_format)
                        .expect("Iterator likely out of bounds")
                        .parse::<i32>()
                        .expect("Failed to parse day field to i8.");
                    counter_format += 1;
                }
                'm' => {
                    month = parts
                        .get(counter_format)
                        .expect("Iterator likely out of bounds")
                        .parse::<i8>()
                        .expect("Failed to parse month field to i8.");
                    counter_format += 1;
                }
                'y' => {
                    year = parts
                        .get(counter_format)
                        .expect("Iterator likely out of bounds")
                        .parse::<i32>()
                        .expect("Failed to parse year field to i16.");
                    counter_format += 1;
                }
                _ => {}
            }
        }
        Date {
            day: day as u8,// the weird parsing of i32 and back to i8 is due to an overflow error that I don't understand, so this is a bandaid
            month: month as u8,
            year: year,
        }
    }
}