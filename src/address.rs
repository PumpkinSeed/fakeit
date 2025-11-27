use crate::data::address;
use crate::misc;
use crate::name;
// use ::std::string::String;

pub struct Info {
    address: String,
    street: String,
    city: String,
    state: String,
    zip: String,
    country: String,
    latitude: f32,
    longitude: f32,
}

pub fn info() -> Info {
    Info {
        address: format!("{}, {}, {} {}", street(), city(), state(), zip()),
        street: street(),
        city: city(),
        state: state(),
        zip: zip(),
        country: country(),
        latitude: latitude(),
        longitude: longitude(),
    }
}

pub fn street() -> String {
    match misc::random::<i64>(1, 2) {
        1 => {
            format!(
                "{} {} {} {}",
                street_number(),
                street_prefix(),
                street_name(),
                street_suffix()
            )
        }
        2 => format!("{} {} {}", street_number(), street_name(), street_suffix()),
        _ => "impossible".to_string(),
    }
}

pub fn street_number() -> String {
    misc::replace_with_numbers(misc::random_data(address::NUMBER).to_string())
}

pub fn street_prefix() -> String {
    misc::random_data(address::STREET_PREFIX).to_string()
}

pub fn street_name() -> String {
    misc::random_data(address::STREET_NAME).to_string()
}

pub fn street_suffix() -> String {
    misc::random_data(address::STREET_SUFFIX).to_string()
}

pub fn city() -> String {
    match misc::random::<i64>(1, 3) {
        1 => format!("{}{}", name::first(), street_suffix()),
        2 => format!("{}{}", name::last(), street_suffix()),
        3 => format!("{} {}", street_prefix(), name::last()),
        _ => "impossible".to_string(),
    }
}

pub fn state() -> String {
    misc::random_data(address::STATE).to_string()
}

pub fn state_abr() -> String {
    misc::random_data(address::STATE_ABR).to_string()
}

pub fn zip() -> String {
    misc::replace_with_numbers(misc::random_data(address::ZIP).to_string())
}

pub fn country() -> String {
    misc::random_data(address::COUNTRY).to_string()
}

pub fn country_abr() -> String {
    misc::random_data(address::COUNTRY_ABR).to_string()
}

pub fn latitude() -> f32 {
    misc::random::<f32>(-90.0, 90.0)
}

pub fn latitude_in_range(min: f32, max: f32) -> f32 {
    if min > max || !(-90.0..=90.0).contains(&min) || !(-90.0..=90.0).contains(&max) {
        return latitude();
    }

    misc::random::<f32>(min, max)
}

pub fn longitude() -> f32 {
    misc::random::<f32>(-180.0, 180.0)
}

pub fn longitude_in_range(min: f32, max: f32) -> f32 {
    if min > max || !(-180.0..=180.0).contains(&min) || !(-180.0..=180.0).contains(&max) {
        return latitude();
    }

    misc::random::<f32>(min, max)
}

#[cfg(test)]
mod tests {
    use crate::address;
    use crate::testify::exec_mes;

    #[test]
    fn street() {
        exec_mes("address::street", || address::street());
    }

    #[test]
    fn street_number() {
        exec_mes("address::street_number", || address::street_number());
    }

    #[test]
    fn street_prefix() {
        exec_mes("address::street_prefix", || address::street_prefix());
    }

    #[test]
    fn street_name() {
        exec_mes("address::street_name", || address::street_name());
    }

    #[test]
    fn street_suffix() {
        exec_mes("address::street_suffix", || address::street_suffix());
    }

    #[test]
    fn city() {
        exec_mes("address::city", || address::city());
    }

    #[test]
    fn state() {
        exec_mes("address::state", || address::state());
    }

    #[test]
    fn state_abr() {
        exec_mes("address::state_abr", || address::state_abr());
    }

    #[test]
    fn zip() {
        exec_mes("address::zip", || address::zip());
    }

    #[test]
    fn country() {
        exec_mes("address::country", || address::country());
    }

    #[test]
    fn country_abr() {
        exec_mes("address::country_abr", || address::country_abr());
    }

    #[test]
    fn latitude() {
        exec_mes("address::latitude", || format!("{}", address::latitude()));
    }

    #[test]
    fn latitude_in_range() {
        exec_mes("address::latitude_in_range", || {
            format!("{}", address::latitude_in_range(-30.0, 30.0))
        });
    }

    #[test]
    fn longitude() {
        exec_mes("address::longitude", || format!("{}", address::longitude()));
    }

    #[test]
    fn longitude_in_range() {
        exec_mes("address::longitude_in_range", || {
            format!("{}", address::longitude_in_range(-30.0, 30.0))
        });
    }
}
