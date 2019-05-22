extern crate rand;

use rand::{thread_rng, Rng};

pub const HASHTAG: &str = "#";
pub const QUESTIONMARK: &str = "?";

pub fn random_data_str(d: &'static [&'static str]) -> &'static str {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(0, d.len());

    d[n]
}

pub fn random_data_str_index(d: &'static [&'static str]) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0, d.len())
}

pub fn random<T: rand::distributions::uniform::SampleUniform>(min: T, max: T) -> T {
    let mut rng = thread_rng();
    rng.gen_range(min, max)
}

pub fn random_usize(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(min, max)
}

pub fn random_float64(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(min, max)
}

pub fn random_float32(min: f32, max: f32) -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(min, max)
}

pub fn replace_with_numbers(s: String) -> String {
    if s == "" {
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
    if s == "" {
        return s;
    }

    let letters: [&'static str; 6] = ["a", "b", "c", "d", "e", "f"];

    let res: Vec<String> = s
        .split("")
        .map(|s| {
            if s == QUESTIONMARK {
                let i = random_usize(0, 5);
                return letters[i].to_string();
            }
            s.to_string()
        })
        .collect();

    res.join("")
}

pub fn replace_with_letter(s: String) -> String {
    if s == "" {
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
                let i = random_usize(0, 5);
                return letters[i].to_string();
            }
            s.to_string()
        })
        .collect();

    res.join("")
}

#[cfg(test)]
mod tests {
    use crate::data::address;
    use crate::misc;

    #[test]
    fn random_data_str() {
        let street1 = misc::random_data_str(address::STREET_NAME);
        let street2 = misc::random_data_str(address::STREET_NAME);
        assert_ne!(street1, street2);
    }

    #[test]
    fn replace_with_numbers() {
        let data1 = misc::replace_with_numbers("####".to_string());
        let data2 = misc::replace_with_numbers("####".to_string());
        assert_ne!(data1, data2);
    }
}
