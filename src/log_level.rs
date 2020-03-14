use crate::misc;
use crate::data::log_level;

pub fn general() -> String { misc::random_data(log_level::GENERAL).to_string() }

pub fn syslog() -> String { misc::random_data(log_level::SYSLOG).to_string() }

pub fn apache() -> String { misc::random_data(log_level::APACHE).to_string() }