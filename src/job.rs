use crate::company;
use crate::data::job;
use crate::misc;

pub struct Info {
    company: String,
    title: String,
    descriptor: String,
    level: String,
}

pub fn info() -> Info {
    Info {
        company: company::company(),
        title: title(),
        descriptor: descriptor(),
        level: level(),
    }
}

pub fn title() -> String {
    misc::random_data(job::TITLE).to_string()
}

pub fn descriptor() -> String {
    misc::random_data(job::DESCRIPTOR).to_string()
}

pub fn level() -> String {
    misc::random_data(job::LEVEL).to_string()
}
