extern crate rand;
extern crate simplerand;

use simplerand::{rand_range, Randomable};
use std::cell::RefCell;
use std::clone::Clone;
use std::time::{SystemTime, UNIX_EPOCH};

pub const HASHTAG: &str = "#";
pub const QUESTIONMARK: &str = "?";

thread_local! {
    static SEEDED_RNG: RefCell<Option<simplerand::Rng>> = RefCell::new(None);
}

/// Set a global seed so all fakeit functions produce deterministic output.
pub fn seed(s: u128) {
    let mut rng = simplerand::Rng::new();
    rng.set_seed(s);
    SEEDED_RNG.with(|cell| {
        *cell.borrow_mut() = Some(rng);
    });
}

/// Clear the global seed, restoring random behavior.
pub fn unseed() {
    SEEDED_RNG.with(|cell| {
        *cell.borrow_mut() = None;
    });
}

pub(crate) fn install_rng(rng: simplerand::Rng) -> Option<simplerand::Rng> {
    SEEDED_RNG.with(|cell| cell.borrow_mut().replace(rng))
}

pub(crate) fn take_rng() -> Option<simplerand::Rng> {
    SEEDED_RNG.with(|cell| cell.borrow_mut().take())
}

pub(crate) fn restore_rng(prev: Option<simplerand::Rng>) {
    SEEDED_RNG.with(|cell| {
        *cell.borrow_mut() = prev;
    });
}

pub trait SeededRandom {
    fn seeded_rand_range(rng: &mut simplerand::Rng, min: Self, max: Self) -> Self;
}

macro_rules! impl_seeded_random_uint {
    ($($t:ty),*) => {
        $(
            impl SeededRandom for $t {
                fn seeded_rand_range(rng: &mut simplerand::Rng, min: Self, max: Self) -> Self {
                    rng.rand_range(min as u128, max as u128) as Self
                }
            }
        )*
    }
}

macro_rules! impl_seeded_random_int {
    ($($t:ty),*) => {
        $(
            impl SeededRandom for $t {
                fn seeded_rand_range(rng: &mut simplerand::Rng, min: Self, max: Self) -> Self {
                    let offset = if min < 0 { (-(min as i128)) as u128 } else { 0 };
                    let adj_min = (min as i128 + offset as i128) as u128;
                    let adj_max = (max as i128 + offset as i128) as u128;
                    let n = rng.rand_range(adj_min, adj_max);
                    (n as i128 - offset as i128) as Self
                }
            }
        )*
    }
}

macro_rules! impl_seeded_random_float {
    ($($t:ty),*) => {
        $(
            impl SeededRandom for $t {
                fn seeded_rand_range(rng: &mut simplerand::Rng, min: Self, max: Self) -> Self {
                    let precision: u128 = 10_000_000;
                    let n = rng.rand_range(0, precision);
                    min + (n as $t / precision as $t) * (max - min)
                }
            }
        )*
    }
}

impl_seeded_random_uint!(u8, u16, u32, u64, usize);
impl_seeded_random_int!(i8, i16, i32, i64, isize);
impl_seeded_random_float!(f32, f64);

pub fn random_data<T: Clone>(d: &[T]) -> T {
    SEEDED_RNG.with(|cell| {
        let mut opt = cell.borrow_mut();
        if let Some(ref mut rng) = *opt {
            let n = rng.rand_range(0, d.len() as u128);
            d[n as usize].clone()
        } else {
            let n = rand_range(0, d.len() as i64);
            d[n as usize].clone()
        }
    })
}

pub fn random_data_index<T>(d: &[T]) -> usize {
    SEEDED_RNG.with(|cell| {
        let mut opt = cell.borrow_mut();
        if let Some(ref mut rng) = *opt {
            rng.rand_range(0, d.len() as u128) as usize
        } else {
            rand_range(0, d.len() as i64) as usize
        }
    })
}

pub fn random<T: Randomable + SeededRandom>(min: T, max: T) -> T {
    SEEDED_RNG.with(|cell| {
        let mut opt = cell.borrow_mut();
        if let Some(ref mut rng) = *opt {
            T::seeded_rand_range(rng, min, max)
        } else {
            rand_range::<T>(min, max)
        }
    })
}

pub fn randn(n: i32) -> i32 {
    SEEDED_RNG.with(|cell| {
        let mut opt = cell.borrow_mut();
        if let Some(ref mut rng) = *opt {
            rng.randn(n as u128) as i32
        } else {
            simplerand::randn(n)
        }
    })
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
    SEEDED_RNG.with(|cell| {
        let mut opt = cell.borrow_mut();
        let end_boundry = s.len() - 1;
        if let Some(ref mut rng) = *opt {
            let n = rng.rand_range(0, end_boundry as u128);
            s[n as usize] as char
        } else {
            let n = rand_range(0, end_boundry as i64);
            s[n as usize] as char
        }
    })
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
