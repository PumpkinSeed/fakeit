extern crate rand;
extern crate simplerand;

use simplerand::{rand_range, Randomable};
use std::clone::Clone;
use std::time::{SystemTime, UNIX_EPOCH};

pub const HASHTAG: &str = "#";
pub const QUESTIONMARK: &str = "?";

pub fn random_data<T: Clone>(d: &[T]) -> T {
    let n = rand_range(0, d.len() as i64);

    d[n as usize].clone()
}

pub fn random_data_index<T>(d: &[T]) -> usize {
    rand_range(0, d.len() as i64) as usize
}

pub fn random<T: Randomable>(min: T, max: T) -> T {
    rand_range::<T>(min, max)
}

pub fn replace_with_numbers(s: String) -> String {
    if s.is_empty() {
        return s;
    }

    let res: Vec<String> = s
        .split("")
        .map(|s| {
            if s == HASHTAG {
                let i = random::<i64>(0, 9);
                return i.to_string();
            }
            s.to_string()
        })
        .collect();

    res.join("")
}

pub fn replace_with_letter_hex(s: String) -> String {
    if s.is_empty() {
        return s;
    }

    let letters: [&'static str; 6] = ["a", "b", "c", "d", "e", "f"];

    let res: Vec<String> = s
        .split("")
        .map(|s| {
            if s == QUESTIONMARK {
                let i = random::<usize>(0, 5);
                return letters[i].to_string();
            }
            s.to_string()
        })
        .collect();

    res.join("")
}

pub fn replace_with_letter(s: String) -> String {
    if s.is_empty() {
        return s;
    }

    let letters: [&'static str; 26] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    let res: Vec<String> = s
        .split("")
        .map(|s| {
            if s == QUESTIONMARK {
                let i = random::<usize>(0, letters.len() - 1);
                return letters[i].to_string();
            }
            s.to_string()
        })
        .collect();

    res.join("")
}

pub fn random_char_from_string(s: &[u8]) -> char {
    let end_boundry = s.len() - 1;
    let n = rand_range(0, end_boundry as i64);
    s[n as usize] as char
}

pub fn current_year() -> u16 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before Unix epoch");
    let secs = now.as_secs();
    (secs / 60 / 60 / 24 / 365) as u16 + 1970
}

#[cfg(test)]
mod tests {
    use crate::data::address;
    use crate::misc;
    use crate::misc::current_year;

    #[test]
    fn test_current_year() {
        println!("{}", current_year());
    }

    #[test]
    fn random_data_test() {
        let mut street1 = misc::random_data(address::STREET_NAME);
        println!("{}", street1);

        street1 = misc::random_data(address::STREET_NAME);
        println!("{}", street1);

        street1 = misc::random_data(address::STREET_NAME);
        println!("{}", street1);
    }

    #[test]
    fn random_data_str() {
        let street1 = misc::random_data(address::STREET_NAME);
        let street2 = misc::random_data(address::STREET_NAME);
        assert_ne!(street1, street2);
    }

    #[test]
    fn replace_with_numbers() {
        let data1 = misc::replace_with_numbers("####".to_string());
        let data2 = misc::replace_with_numbers("####".to_string());
        assert_ne!(data1, data2);
    }
}
