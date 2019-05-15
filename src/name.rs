use crate::data::person;
use crate::misc;
use ::std::string::String;

pub fn full() -> String {
    format!("{} {}", first(), last())
}

pub fn first() -> String {
    misc::random_data_str(person::FIRST).to_string()
}

pub fn last() -> String {
    misc::random_data_str(person::LAST).to_string()
}

pub fn prefix() -> String {
    misc::random_data_str(person::PREFIX).to_string()
}

pub fn suffix() -> String {
    misc::random_data_str(person::SUFFIX).to_string()
}
