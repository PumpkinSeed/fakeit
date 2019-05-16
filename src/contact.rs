use crate::data::contact;
use crate::data::internet;
use crate::misc;
use crate::name;
use ::std::string::String;

pub struct ContactInfo {
    phone: String,
    email: String,
}

pub fn contact() -> ContactInfo {
    ContactInfo {
        phone: phone_formatted(),
        email: email(),
    }
}

pub fn phone() -> String {
    misc::replace_with_numbers("##########".to_string())
}

pub fn phone_formatted() -> String {
    misc::replace_with_numbers(misc::random_data_str(contact::PHONE).to_string())
}

pub fn email() -> String {
    format!(
        "{}{}@{}.{}",
        name::first(),
        name::last(),
        name::last(),
        misc::random_data_str(internet::DOMAIN_SUFFIX).to_string()
    )
    .to_lowercase()
}
