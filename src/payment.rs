extern crate chrono;

use crate::data::payment;
use crate::misc;
use chrono::{Datelike, Utc};

pub struct CreditCard {
    type_of: String,
    number: String,
    exp: String,
    cvv: String,
}

pub fn credit_card() -> CreditCard {
    CreditCard {
        type_of: credit_card_type(),
        number: credit_card_number(),
        exp: credit_card_exp(),
        cvv: credit_card_cvv(),
    }
}

pub fn credit_card_type() -> String {
    misc::random_data(payment::CARD_TYPE).to_string()
}

pub fn credit_card_number() -> String {
    misc::replace_with_numbers(misc::random_data(payment::NUMBER).to_string())
}

pub fn credit_card_luhn_number() -> String {
    // @TODO
    return String::from("");
}

pub fn credit_card_exp() -> String {
    let current_year = Utc::now().year() - 2000;
    let month = misc::random(1, 12);
    if month < 10 {
        format!(
            "{}/{}",
            format!("0{}", month).as_str(),
            current_year + misc::random(1, 10)
        )
    } else {
        format!(
            "{}/{}",
            format!("{}", month).as_str(),
            current_year + misc::random(1, 10)
        )
    }
}

pub fn credit_card_cvv() -> String {
    misc::replace_with_numbers("###".to_string())
}

#[cfg(test)]
mod tests {
    use crate::payment;
    use crate::test_helper;

    #[test]
    fn credit_card_type() {
        let data1 = payment::credit_card_type();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn credit_card_number() {
        let data1 = payment::credit_card_number();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn credit_card_exp() {
        let data1 = payment::credit_card_exp();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn credit_card_cvv() {
        let data1 = payment::credit_card_cvv();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}