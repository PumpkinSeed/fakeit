use crate::data::address;
use crate::misc;
use crate::name;
use ::std::string::String;

pub struct AddressInfo {
    address: String,
    street: String,
    city: String,
    state: String,
    zip: String,
    country: String,
    latitude: f32,
    longitude: f32,
}

pub fn address() -> AddressInfo {
    AddressInfo {
        address: format!("{} {} {} {}", street(), city(), state(), zip()),
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
    misc::replace_with_numbers(misc::random_data_str(address::NUMBER).to_string())
}

pub fn street_prefix() -> String {
    misc::random_data_str(address::STREET_PREFIX).to_string()
}

pub fn street_name() -> String {
    misc::random_data_str(address::STATE).to_string()
}

pub fn street_suffix() -> String {
    misc::random_data_str(address::STREET_SUFFIX).to_string()
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
    misc::random_data_str(address::STATE).to_string()
}

pub fn state_abr() -> String {
    misc::random_data_str(address::STATE_ABR).to_string()
}

pub fn zip() -> String {
    misc::replace_with_numbers(misc::random_data_str(address::ZIP).to_string())
}

pub fn country() -> String {
    misc::random_data_str(address::COUNTRY).to_string()
}

pub fn country_abr() -> String {
    misc::random_data_str(address::COUNTRY_ABR).to_string()
}

pub fn latitude() -> f32 {
    misc::random_float32(-90.0, 90.0)
}

pub fn latitude_in_range(min: f32, max: f32) -> f32 {
    if min > max || min < -90.0 || min > 90.0 || max < -90.0 || max > 90.0 {
        return latitude();
    }

    misc::random_float32(min, max)
}

pub fn longitude() -> f32 {
    misc::random_float32(-180.0, 180.0)
}

pub fn longitude_in_range(min: f32, max: f32) -> f32 {
    if min > max || min < -180.0 || min > 180.0 || max < -180.0 || max > 180.0 {
        return latitude();
    }

    misc::random_float32(min, max)
}

#[cfg(test)]
mod tests {
    use crate::address;

    #[test]
    fn zip() {
        let zip = address::zip();
        println!("{}", zip);
    }
}
