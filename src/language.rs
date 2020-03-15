use crate::misc;
use crate::data::language;

pub fn random() -> String { misc::random_data(language::LONG).to_string() }

pub fn abbreviation() -> String { misc::random_data(language::SHORT).to_string() }

pub fn programming() -> String { misc::random_data(language::PROGRAMMING).to_string() }