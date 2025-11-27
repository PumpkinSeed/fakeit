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
    use crate::testify::exec_mes;

    #[test]
    fn short() {
        exec_mes("currency::short", currency::short);
    }

    #[test]
    fn long() {
        exec_mes("currency::long", currency::long);
    }

    #[test]
    fn price() {
        exec_mes("currency::short", || {
            format!("{}", currency::price(1_f64, 123_f64))
        });
    }
}
