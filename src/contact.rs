use crate::data::contact;
use crate::data::internet;
use crate::misc;
use crate::name;
use ::std::string::String;

pub struct Info {
    phone: String,
    email: String,
}

pub fn info() -> Info {
    Info {
        phone: phone_formatted(),
        email: email(),
    }
}

pub fn phone() -> String {
    misc::replace_with_numbers("##########".to_string())
}

pub fn phone_formatted() -> String {
    misc::replace_with_numbers(misc::random_data(contact::PHONE).to_string())
}

pub fn email() -> String {
    format!(
        "{}{}@{}.{}",
        name::first(),
        name::last(),
        name::last(),
        misc::random_data(internet::DOMAIN_SUFFIX)
    )
    .to_lowercase()
}

#[cfg(test)]
mod tests {
    use crate::contact;
    use crate::testify::exec_mes;

    #[test]
    fn phone() {
        exec_mes("contact::phone", contact::phone);
    }

    #[test]
    fn phone_formatted() {
        exec_mes("contact::phone_formatted", contact::phone_formatted);
    }

    #[test]
    fn email() {
        exec_mes("contact::email", contact::email);
    }
}
