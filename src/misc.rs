extern crate rand;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::clone::Clone;

pub const HASHTAG: &str = "#";
pub const QUESTIONMARK: &str = "?";

pub fn random_data<T: Clone>(d: &[T]) -> T {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(0, d.len());

    let res = d[n].clone();
    res
}

pub fn random_data_index<T>(d: &[T]) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0, d.len())
}

pub fn random<T: rand::distributions::uniform::SampleUniform>(min: T, max: T) -> T {
    let mut rng = thread_rng();
    rng.gen_range(min, max)
}

pub fn replace_with_numbers(s: String) -> String {
    if s == String::from("") {
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
    if s == String::from("") {
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
    if s == String::from("") {
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
                let i = random::<usize>(0, 5);
                return letters[i].to_string();
            }
            s.to_string()
        })
        .collect();

    res.join("")
}

pub fn random_char_from_string(s: &[u8]) -> char {
    let mut r = thread_rng();
    s.choose(&mut r).cloned().unwrap().into()
}

#[cfg(test)]
mod tests {
    use crate::data::address;
    use crate::misc;

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
