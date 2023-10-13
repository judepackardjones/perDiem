use crate::types::*;

impl x for String {
    fn as_Date(&self) -> Date {
        todo!()
    }
}

impl Date {
    fn to_string(&self, date_format: &mut str, separator: &char) -> Result<String, &str> {
        let date_format = date_format.to_ascii_lowercase();
        match (date_format.contains("dd"), date_format.contains("mm")) {
        (true, true) => {
            let date_format = date_format.replace("dd", &format!("{:>2}", self.day));
            let date_format = date_format.replace("mm", &format!("{:>2}", self.month));
        },
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
                    let date_format = date_format.replace("yy", self.last_two_digits_year().to_string().as_str());
                } else {
                    return Err("failed to find suitable year formatting.");
                }
            }
        }
        Ok(date_format.to_string())
    }
}

fn with_separators(format: &mut str, separator: &char) {

}
