use crate::data::payment;
use crate::misc;
use simplerand::randn;

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

fn gen_random_num(length:usize) -> Vec<i32> {
    let mut nums=vec![0i32;length];
    for x in &mut nums {
        *x=randn(9);
    }
    nums
}

pub fn credit_card_luhn_number() -> String {
    let mii=randn(9);//MII (Major Industry Identifier)
    let nums=gen_random_num(14);
    let iin=[mii.to_string(),nums.iter().map(ToString::to_string).collect()].join("");
    let mut total=0;

    for (i,val) in iin.chars().rev().enumerate() {
        if i%2!=0 {
            total+=val.to_digit(10).expect("error");
        }
        else {
            let double=val.to_digit(10).expect("error")*2;
            let digits=double.to_string();
            if digits.len()>1 {
                for j in digits.chars() {
                    total+=j.to_digit(10).expect("error");
                }
            }
            else {
                total+=double;
            }
        }  
    }
    let check=10-(total % 10)%10;
    format!("{iin}{check}")
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
