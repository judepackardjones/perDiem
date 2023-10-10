use date_utils::types::*;

#[cfg(test)]
mod tests {

    #[test]
    fn date_to_weekday() {
        assert_eq!(Date{ day: 7, month: 10,  year: 1500}.weekday().unwrap(), "Sunday");
    }
    #[test]
    fn isLeapYear() {
    assert_eq!(Date{ day: 7, month: 10,  year: 1900}.isLeapYear(), false);
    }
}
