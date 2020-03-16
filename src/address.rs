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
    use crate::test_helper;

    #[test]
    fn street() {
        let data1 = address::street();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn street_number() {
        let data1 = address::street_number();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn street_prefix() {
        let data1 = address::street_prefix();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn street_name() {
        let data1 = address::street_name();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn street_suffix() {
        let data1 = address::street_suffix();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn city() {
        let data1 = address::city();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn state() {
        let data1 = address::state();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn state_abr() {
        let data1 = address::state_abr();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn zip() {
        let data1 = address::zip();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn country() {
        let data1 = address::country();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn country_abr() {
        let data1 = address::country_abr();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn latitude() {
        let data1 = address::latitude();
        assert_ne!(data1, 0 as f32);
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn latitude_in_range() {
        let data1 = address::latitude_in_range(-30.0, 30.0);
        assert_ne!(data1, 0 as f32);
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn longitude() {
        let data1 = address::longitude();
        assert_ne!(data1, 0 as f32);
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn longitude_in_range() {
        let data1 = address::longitude_in_range(-30.0, 30.0);
        assert_ne!(data1, 0 as f32);
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}
