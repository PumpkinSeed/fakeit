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
    misc::random::<i32>(0, Utc::now().year()).to_string()
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
    misc::random_data_str(datetime::TEXT).to_string()
}

pub fn timezone_full() -> String {
    misc::random_data_str(datetime::FULL).to_string()
}

pub fn timezone_abv() -> String {
    misc::random_data_str(datetime::ABR).to_string()
}

pub fn timezone_offset() -> String {
    misc::random_data_str(datetime::OFFSET).to_string()
}

pub fn date_range(min: String, max: String) -> DateTime<Utc> {
    // RFC3339
    let min_nano = DateTime::parse_from_rfc3339(&min)
        .unwrap()
        .timestamp_nanos();
    let max_nano = DateTime::parse_from_rfc3339(&max)
        .unwrap()
        .timestamp_nanos();

    let ns = misc::random::<i64>(min_nano, max_nano);
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, ns as u32), Utc)
}

pub fn date() -> DateTime<Utc> {
    date_range(
        "1970-01-01T16:39:57-08:00".to_string(),
        Utc::now().to_rfc3339(),
    )
}
