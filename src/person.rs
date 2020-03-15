use crate::misc;
use crate::job;
use crate::address;
use crate::contact;
use crate::payment;
use crate::name;
use crate::image;
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

pub fn person() -> Info {
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