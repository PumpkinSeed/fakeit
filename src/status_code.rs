use crate::data::status_code;
use crate::misc;

pub fn simple() -> i16 { misc::random_data(status_code::SIMPLE) }

pub fn general() -> i16 { misc::random_data(status_code::GENERAL) }