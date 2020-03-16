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
            return format!(
                "{} {} {} {}",
                street_number(),
                street_prefix(),
                street_name(),
                street_suffix()
            )
        }
        2 => return format!("{} {} {}", street_number(), street_name(), street_suffix()),
        _ => format!("impossible"),
    }
}

pub fn street_number() -> String {
    misc::replace_with_numbers(misc::random_data(address::NUMBER).to_string())
}

pub fn street_prefix() -> String {
    misc::random_data(address::STREET_PREFIX).to_string()
}

pub fn street_name() -> String {
    misc::random_data(address::STATE).to_string()
}

pub fn street_suffix() -> String {
    misc::random_data(address::STREET_SUFFIX).to_string()
}

pub fn city() -> String {
    match misc::random::<i64>(1, 3) {
        1 => return format!("{}{}", name::first(), street_suffix()),
        2 => return format!("{}{}", name::last(), street_suffix()),
        3 => return format!("{} {}", street_prefix(), name::last()),
        _ => format!("impossible"),
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
    if min > max || min < -90.0 || min > 90.0 || max < -90.0 || max > 90.0 {
        return latitude();
    }

    misc::random::<f32>(min, max)
}

pub fn longitude() -> f32 {
    misc::random::<f32>(-180.0, 180.0)
}

pub fn longitude_in_range(min: f32, max: f32) -> f32 {
    if min > max || min < -180.0 || min > 180.0 || max < -180.0 || max > 180.0 {
        return latitude();
    }

    misc::random::<f32>(min, max)
}

#[cfg(test)]
mod tests {
    use crate::address;

    #[test]
    fn street() {
        let street1 = address::street();
        let street2 = address::street();
        assert_ne!(street1, street2);
    }

    #[test]
    fn street_number() {
        let street_number1 = address::street_number();
        let street_number2 = address::street_number();
        assert_ne!(street_number1, street_number2);
    }

    #[test]
    fn street_prefix() {
        let street_prefix1 = address::street_prefix();
        let street_prefix2 = address::street_prefix();
        assert_ne!(street_prefix1, street_prefix2);
    }

    #[test]
    fn street_name() {
        let street_name1 = address::street_name();
        let street_name2 = address::street_name();
        assert_ne!(street_name1, street_name2);
    }

    #[test]
    fn street_suffix() {
        let street_suffix1 = address::street_suffix();
        let street_suffix2 = address::street_suffix();
        assert_ne!(street_suffix1, street_suffix2);
    }

    #[test]
    fn city() {
        let city1 = address::city();
        let city2 = address::city();
        assert_ne!(city1, city2);
    }

    #[test]
    fn state() {
        let state1 = address::state();
        let state2 = address::state();
        assert_ne!(state1, state2);
    }

    #[test]
    fn state_abr() {
        let state_abr1 = address::state_abr();
        let state_abr2 = address::state_abr();
        assert_ne!(state_abr1, state_abr2);
    }

    #[test]
    fn zip() {
        let zip1 = address::zip();
        let zip2 = address::zip();
        assert_ne!(zip1, zip2);
    }

    #[test]
    fn country() {
        let country1 = address::country();
        let country2 = address::country();
        assert_ne!(country1, country2);
    }

    #[test]
    fn country_abr() {
        let country_abr1 = address::country_abr();
        let country_abr2 = address::country_abr();
        assert_ne!(country_abr1, country_abr2);
    }

    #[test]
    fn latitude() {
        let latitude1 = address::latitude();
        let latitude2 = address::latitude();
        assert_ne!(latitude1, latitude2);
    }

    #[test]
    fn latitude_in_range() {
        let latitude_in_range1 = address::latitude_in_range(-30.0, 30.0);
        let latitude_in_range2 = address::latitude_in_range(-30.0, 30.0);
        assert_ne!(latitude_in_range1, latitude_in_range2);
    }

    #[test]
    fn longitude() {
        let longitude1 = address::longitude();
        let longitude2 = address::longitude();
        assert_ne!(longitude1, longitude2);
    }

    #[test]
    fn longitude_in_range() {
        let longitude_in_range1 = address::longitude_in_range(-30.0, 30.0);
        let longitude_in_range2 = address::longitude_in_range(-30.0, 30.0);
        assert_ne!(longitude_in_range1, longitude_in_range2);
    }
}
