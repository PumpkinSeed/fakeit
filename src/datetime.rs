extern crate chrono;

use crate::data::datetime;
use crate::misc;
use chrono::{DateTime, Datelike, NaiveDateTime, Utc};

pub fn month() -> String {
    misc::random::<i8>(1, 12).to_string()
}

pub fn day() -> String {
    misc::random::<i8>(1, 28).to_string()
}

pub fn week_day() -> String {
    misc::random::<i8>(0, 6).to_string()
}

pub fn year() -> String {
    misc::random::<i32>(1980, Utc::now().year()).to_string()
}

pub fn hour() -> String {
    misc::random::<i8>(0, 23).to_string()
}

pub fn minute() -> String {
    misc::random::<i8>(0, 59).to_string()
}

pub fn second() -> String {
    misc::random::<i8>(0, 59).to_string()
}

pub fn nanosecond() -> String {
    misc::random::<i64>(0, 999999999).to_string()
}

pub fn timezone() -> String {
    misc::random_data(datetime::TEXT).to_string()
}

pub fn timezone_full() -> String {
    misc::random_data(datetime::FULL).to_string()
}

pub fn timezone_abv() -> String {
    misc::random_data(datetime::ABR).to_string()
}

pub fn timezone_offset() -> String {
    misc::random_data(datetime::OFFSET).to_string()
}

pub fn date_range(min: String, max: String) -> DateTime<Utc> {
    // RFC3339
    let min_nano = DateTime::parse_from_rfc3339(&min)
        .unwrap()
        .timestamp_nanos();
    let max_nano = DateTime::parse_from_rfc3339(&max)
        .unwrap()
        .timestamp_nanos();
    let ns = misc::random(min_nano, max_nano-10_000_000_000);
    let secs = (ns / 1_000_000_000) as i64;
    let nsecs= (ns - (secs * 1_000_000_000)) as u32;
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(secs, nsecs as u32), Utc)
}

pub fn date() -> DateTime<Utc> {
    date_range(
        "1970-01-01T16:39:57-08:00".to_string(),
        Utc::now().to_rfc3339(),
    )
}

#[cfg(test)]
mod tests {
    use crate::datetime;
    use crate::test_helper;

    #[test]
    fn timezone() {
        let data1 = datetime::timezone();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn timezone_full() {
        let data1 = datetime::timezone_full();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn timezone_abv() {
        let data1 = datetime::timezone_abv();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn timezone_offset() {
        let data1 = datetime::timezone_offset();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn date() {
        let data1 = datetime::date();
        let data2 = datetime::date();
        assert_ne!(data1, data2);
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}