use crate::data::datetime;
use crate::misc;
use std::str::from_utf8;

#[derive(Default, Debug, PartialEq)]
pub struct DateTime {
    pub secs: i64,
    pub nsecs: u32,
}

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
    misc::random::<i32>(1980, misc::current_year() as i32).to_string()
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

pub fn date_range(min: String, max: String) -> DateTime {
    // RFC3339
    let min_nano = parse_from_rfc3339(min).unwrap();
    let max_nano = parse_from_rfc3339(max).unwrap();
    let ns = misc::random(min_nano, max_nano - 10_000_000_000);
    let secs = (ns / 1_000_000_000) as i64;
    let mut nsecs = (ns - (secs * 1_000_000_000)) as u32;

    // This case will cause the `NaiveDateTime::from_timestamp` function to panic.
    // So we just roll it back to the maximum allowed value.
    if nsecs >= 2_000_000_000 {
        nsecs = 1_999_999_999;
    }

    DateTime { secs, nsecs }

    // let datetime = NaiveDateTime::from_timestamp_opt(secs, nsecs as u32)
    //     .expect("invalid or out-of-range datetime");
    // DateTime::<Utc>::from_utc(datetime, Utc)
}

pub fn date() -> DateTime {
    date_range(
        "1970-01-01T16:39:57-08:00".to_string(),
        "2023-09-14T13:21:06-00:00".to_string(),
    )
}

#[derive(Debug)]
struct InternalDateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    min: u8,
    sec: u8,
}

fn parse_from_rfc3339(str: String) -> Result<i64, String> {
    let bytes = str.as_bytes();

    let dt = InternalDateTime {
        year: number_parse::<u16>(&bytes[0..4])?,
        month: number_parse::<u8>(&bytes[5..7])?,
        day: number_parse::<u8>(&bytes[8..10])?,
        hour: number_parse::<u8>(&bytes[11..13])?,
        min: number_parse::<u8>(&bytes[14..16])?,
        sec: number_parse::<u8>(&bytes[17..19])?,
    };

    let mut secs: i64 = 0;
    secs += dt.sec as i64;
    secs += dt.min as i64 * 60;
    secs += dt.hour as i64 * 3_600;
    secs += dt.day as i64 * 86_400;
    secs += dt.month as i64 * 2_629_746;
    secs += dt.year as i64 * 31_556_952;

    let epoch_secs = 62_169_911_586;
    let sec_to_nano = 1_000_000_000;
    let timestamp = secs - epoch_secs;

    Ok(timestamp * sec_to_nano)
}

fn number_parse<T: std::str::FromStr>(s: &[u8]) -> Result<T, String> {
    let s_str = match from_utf8(s) {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    };
    match s_str.parse::<T>() {
        Ok(y) => Ok(y),
        Err(_e) => Err(String::from("Parsing the str to T failed")),
    }
}

#[cfg(test)]
mod tests {
    use crate::datetime;
    use crate::datetime::parse_from_rfc3339;
    use crate::testify::exec_mes;

    #[test]
    fn timezone() {
        exec_mes("datetime::timezone", || datetime::timezone());
    }

    #[test]
    fn timezone_full() {
        exec_mes("datetime::timezone_full", || datetime::timezone_full());
    }

    #[test]
    fn timezone_abv() {
        exec_mes("datetime::timezone_abv", || datetime::timezone_abv());
    }

    #[test]
    fn timezone_offset() {
        exec_mes("datetime::timezone_offset", || datetime::timezone_offset());
    }

    #[test]
    fn date() {
        let data1 = datetime::date();
        let data2 = datetime::date();
        assert_ne!(data1, data2);
    }

    #[test]
    fn test_parse() {
        let min = "1970-02-02T16:16:34-00:00".to_string();
        let min_nano_internal = parse_from_rfc3339(min).unwrap();
        println!("Own:    {}", min_nano_internal);
    }

    #[test]
    fn test_date() {
        let date1 = datetime::date();
        println!("{:?}", date1);
    }
}
