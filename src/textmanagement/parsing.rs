use crate::types::*;

impl x for String {
    fn as_Date(&self) -> Date {
        todo!()
    }
}

impl y for &mut str {
        fn with_separators(&self, separator: &char) -> String {
        let mut result: String = String::new();
        let mut counter: i8 = 0;
        let mut index: usize = 0;
        for i in self.chars() {
            match counter {
                0 => {
                    result.push(i);
                }
                1 => {
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
                _ => {
                    panic!("SOMETHING TERRIBLY WRONG")
                }
            }
            if counter == 1 {
                counter = 0;
                continue;
            }
            index += 1;
            counter += 1;
        }
        if result.chars().nth(result.len() - 1).expect("Failed to unwrap Option<char> to char") == '/' {
            result.remove(result.len() - 1);
        }
        result
    }
    
}

impl Date {
    fn to_string(&self, date_format: &mut str, separator: &char) -> Result<String, &str> {
        let date_format = date_format.with_separators(separator);
        let date_format = date_format.to_ascii_lowercase();
        match (date_format.contains("dd"), date_format.contains("mm")) {
            (true, true) => {
                let date_format = date_format.replace("dd", &format!("{:>2}", self.day));
                let date_format = date_format.replace("mm", &format!("{:>2}", self.month));
            }
            (false, true) => return Err("Failed to find dd in format structure"),
            (true, false) => return Err("Failed to find mm in format structure"),
            (false, false) => return Err("Failed to find mm and dd in format structure"),
        }
        match date_format.contains("yyyy") {
            true => {
                let format = date_format.replace("yyyy", self.year.to_string().as_str());
            }
            false => {
                if date_format.contains("yy") {
                    let date_format =
                        date_format.replace("yy", self.last_two_digits_year().to_string().as_str());
                } else {
                    return Err("failed to find suitable year formatting.");
                }
            }
        }
        Ok(date_format.to_string())
    }
}
