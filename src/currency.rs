extern crate math;

use crate::data::currency;
use crate::misc;
use math::round;

pub struct Info {
    short: String,
    long: String,
}

pub fn compact() -> Info {
    let index = misc::random_data_index(currency::SHORT);
    Info {
        short: currency::SHORT[index].to_string(),
        long: currency::LONG[index].to_string(),
    }
}

pub fn short() -> String {
    misc::random_data(currency::SHORT).to_string()
}

pub fn long() -> String {
    misc::random_data(currency::LONG).to_string()
}

pub fn price(min: f64, max: f64) -> f64 {
    round::floor(misc::random::<f64>(min, max), 2)
}

#[cfg(test)]
mod tests {
    use crate::currency;
    use crate::test_helper;

    #[test]
    fn short() {
        let data1 = currency::short();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn long() {
        let data1 = currency::long();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn price() {
        let data1 = currency::price(1 as f64, 123 as f64);
        assert_ne!(data1, 0 as f64);
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}