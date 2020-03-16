use crate::address;
use crate::contact;
use crate::image;
use crate::job;
use crate::misc;
use crate::name;
use crate::payment;
use std::ops::Add;

pub struct Info {
    first_name: String,
    last_name: String,
    gender: String,
    ssn: String,
    image: String,
    job: job::Info,
    address: address::Info,
    contact: contact::Info,
    credit_card: payment::CreditCard,
}

pub fn info() -> Info {
    Info {
        first_name: name::first(),
        last_name: name::last(),
        gender: gender(),
        ssn: ssn(),
        image: image::url(300, 300).add("/people"),
        job: job::info(),
        address: address::info(),
        contact: contact::info(),
        credit_card: payment::credit_card(),
    }
}

pub fn ssn() -> String {
    format!("{}", misc::random(100000000, 999999999))
}

pub fn gender() -> String {
    match misc::random(1, 2) {
        1 => "male".to_string(),
        _ => "female".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::person;
    use crate::test_helper;

    #[test]
    fn ssn() {
        let data1 = person::ssn();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn gender() {
        let data1 = person::gender();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}