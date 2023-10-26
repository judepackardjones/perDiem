use crate::types::*;

impl Date {
    ///Converts a Date object to a String. Give date format something like ddmmyyyy or ddmmyy(for last two digits of year). And separator to output in the format given.
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
