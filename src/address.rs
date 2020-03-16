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
        let street = address::street();
        println!("street: {}", street);
    }

    #[test]
    fn street_number() {
        let street_number = address::street_number();
        println!("street_number: {}", street_number);
    }

    #[test]
    fn street_prefix() {
        let street_prefix = address::street_prefix();
        println!("street_prefix: {}", street_prefix);
    }

    #[test]
    fn street_name() {
        let street_name = address::street_name();
        println!("street_name: {}", street_name);
    }

    #[test]
    fn street_suffix() {
        let street_suffix = address::street_suffix();
        println!("street_suffix: {}", street_suffix);
    }

    #[test]
    fn city() {
        let city = address::city();
        println!("city: {}", city);
    }

    #[test]
    fn state() {
        let state = address::state();
        println!("state: {}", state);
    }

    #[test]
    fn state_abr() {
        let state_abr = address::state_abr();
        println!("state_abr: {}", state_abr);
    }

    #[test]
    fn zip() {
        let zip = address::zip();
        println!("zip: {}", zip);
    }

    #[test]
    fn country() {
        let country = address::country();
        println!("country: {}", country);
    }

    #[test]
    fn country_abr() {
        let country_abr = address::country_abr();
        println!("country_abr: {}", country_abr);
    }

    #[test]
    fn latitude() {
        let latitude = address::latitude();
        println!("latitude: {}", latitude);
    }

    #[test]
    fn latitude_in_range() {
        let latitude_in_range = address::latitude_in_range(-30.0, 30.0);
        println!("latitude_in_range: {}", latitude_in_range);
    }

    #[test]
    fn longitude() {
        let longitude = address::longitude();
        println!("longitude: {}", longitude);
    }

    #[test]
    fn longitude_in_range() {
        let longitude_in_range = address::longitude_in_range(-30.0, 30.0);
        println!("longitude_in_range: {}", longitude_in_range);
    }
}
