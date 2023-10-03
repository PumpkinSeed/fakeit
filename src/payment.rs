use crate::data::payment;
use crate::misc;

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
    let mut cc_str = credit_card_number();
    let mut cc_num = cc_str
        .parse::<u64>()
        .expect("generated bad credit card number");

    // iterate until you hit a cc number that passes the luhn check
    while !is_luhn(&cc_str) {
        cc_num += 1;
        cc_str = cc_num.to_string();
    }

    cc_str
}

// is_luhn check is used for checking if credit card is a valid luhn card
fn is_luhn(cc_str: &String) -> bool {
    let parity = cc_str.len() & 1;
    let mut sum = 0;
    for (i, ch) in cc_str.chars().enumerate() {
        let Some(digit) = ch.to_digit(10) else {
            return false;
        };

        if i & 1 != parity {
            sum += digit;
        } else if digit > 4 {
            sum += digit * 2 - 9;
        } else {
            sum += digit * 2;
        }
    }
    sum % 10 == 0
}

pub fn credit_card_exp() -> String {
    let current_year = misc::current_year() as i32 - 2000;
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
    use crate::testify::exec_mes;

    #[test]
    fn credit_card_type() {
        exec_mes("payment::credit_card_type", || payment::credit_card_type());
    }

    #[test]
    fn credit_card_number() {
        exec_mes("payment::credit_card_number", || {
            payment::credit_card_number()
        });
    }

    #[test]
    fn is_luhn() {
        exec_mes("payment::is_luhn", || {
            (if payment::is_luhn(&"4716685826369360".to_string()) {
                "luhn"
            } else {
                ""
            })
            .to_string()
        });
    }

    #[test]
    fn credit_card_luhn_number() {
        exec_mes("payment::credit_card_luhn_number", || {
            payment::credit_card_luhn_number()
        });
    }

    #[test]
    fn credit_card_exp() {
        exec_mes("payment::credit_card_exp", || payment::credit_card_exp());
    }

    #[test]
    fn credit_card_cvv() {
        exec_mes("payment::credit_card_cvv", || payment::credit_card_cvv());
    }
}
