use chrono::{Utc, Date, NaiveDate};

pub fn from_string(date: &str) -> Result<Date<Utc>, &'static str> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map(|d| Date::from_utc(d, Utc))
        .map_err(|_| "Date parse error")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn success_parsing() {
        let result = from_string("2020-01-01");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Utc.ymd(2020, 1, 1)
        );
    }

    #[test]
    fn fail_due_to_invalid_format() {
        let result = from_string("2020/01/01");
        assert!(result.is_err());
    }

    #[test]
    fn fail_due_to_invalid_date() {
        let result = from_string("2021-02-29");
        assert!(result.is_err());
    }
}
