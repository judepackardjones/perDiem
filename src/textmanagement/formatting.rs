use crate::types::*;

impl x for String {
    fn as_Date(&self) -> Date {
        todo!()
    }
}

impl Date {
    pub fn to_string(&self, date_format: &str, separator: &char) -> Result<String, &str> {
        let mut rtn_str = date_format.with_separators(separator);
        rtn_str = rtn_str.to_ascii_lowercase();
        match (rtn_str.contains("dd"), rtn_str.contains("mm")) {
            (true, true) => {
                rtn_str = rtn_str.replace("dd", &format!("{:02}", self.day));
                rtn_str = rtn_str.replace("mm", &format!("{:02}", self.month));
            }
            (false, true) => return Err("Failed to find dd in format structure"),
            (true, false) => return Err("Failed to find mm in format structure"),
            (false, false) => return Err("Failed to find mm and dd in format structure"),
        }
        match rtn_str.contains("yyyy") {
            true => {
                rtn_str = rtn_str.replace("yyyy", self.year.to_string().as_str());
            }
            false => {
                if rtn_str.contains("yy") {
                    rtn_str =
                        rtn_str.replace("yy", self.last_two_digits_year().to_string().as_str());
                } else {
                    println!("{rtn_str}");
                    return Err("failed to find suitable year formatting.");
                }
            }
        }
        Ok(rtn_str.to_string())
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
