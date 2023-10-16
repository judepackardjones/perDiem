use crate::types::*;
use regex::Regex;

impl x for String {
    fn as_Date(&self, format: &str) -> Date {
        if format == "" {
            format = "ddmmyyyy"
        }
        let regex_build: &str = 
        let re: Regex = Regex::new(r"").unwrap();


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
