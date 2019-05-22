use crate::data::company;
use crate::misc;
use crate::name;
use ::std::string::String;

pub fn company() -> String {
    match misc::random::<i64>(1, 3) {
        1 => return format!("{}, {} and {}", name::last(), name::last(), name::last()),
        2 => return format!("{}-{}", name::last(), name::last()),
        3 => return format!("{} {}", name::last(), company_suffix()),
        _ => format!("impossible"),
    }
}

pub fn company_suffix() -> String {
    misc::random_data_str(company::SUFFIX).to_string()
}

pub fn buzzword() -> String {
    misc::random_data_str(company::BUZZWORDS).to_string()
}

pub fn bs() -> String {
    misc::random_data_str(company::BS).to_string()
}
