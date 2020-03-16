use crate::company;
use crate::data::internet;
use crate::data::person;
use crate::job;
use crate::misc;

pub fn domain_name() -> String {
    format!(
        "{}{}.{}",
        job::descriptor().to_lowercase(),
        company::bs().to_lowercase(),
        domain_suffix()
    )
}

pub fn http_method() -> String {
    misc::random_data(internet::HTTP_METHOD).to_string()
}

pub fn domain_suffix() -> String {
    misc::random_data(internet::DOMAIN_SUFFIX).to_string()
}

pub fn ipv4_address() -> String {
    format!(
        "{}.{}.{}.{}",
        misc::random::<i16>(2, 254),
        misc::random::<i16>(2, 254),
        misc::random::<i16>(2, 254),
        misc::random::<i16>(2, 254),
    )
}

pub fn ipv6_address() -> String {
    let num: i64 = 65536;
    misc::random::<i64>(0, num);
    format!(
        "2001:cafe:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
        misc::random::<i64>(0, num),
        misc::random::<i64>(0, num),
        misc::random::<i64>(0, num),
        misc::random::<i64>(0, num),
        misc::random::<i64>(0, num),
        misc::random::<i64>(0, num),
    )
}

pub fn mac_address() -> String {
    let num: i16 = 255;
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        misc::random(0, num),
        misc::random(0, num),
        misc::random(0, num),
        misc::random(0, num),
        misc::random(0, num),
        misc::random(0, num),
    )
}

pub fn username() -> String {
    format!(
        "{}{}",
        misc::random_data(person::LAST).to_string(),
        misc::replace_with_numbers("####".to_string()),
    )
}

#[cfg(test)]
mod tests {
    use crate::internet;

    #[test]
    fn domain_name() {
        let data1 = internet::domain_name();
        let data2 = internet::domain_name();
        assert_ne!(data1, data2);
    }

    #[test]
    fn http_method() {
        let data1 = internet::http_method();
        let data2 = internet::http_method();
        assert_ne!(data1, data2);
    }

    #[test]
    fn domain_suffix() {
        let data1 = internet::domain_suffix();
        let data2 = internet::domain_suffix();
        assert_ne!(data1, data2);
    }

    #[test]
    fn ipv4_address() {
        let data1 = internet::ipv4_address();
        let data2 = internet::ipv4_address();
        assert_ne!(data1, data2);
    }

    #[test]
    fn ipv6_address() {
        let data1 = internet::ipv6_address();
        let data2 = internet::ipv6_address();
        assert_ne!(data1, data2);
    }

    #[test]
    fn mac_address() {
        let data1 = internet::mac_address();
        let data2 = internet::mac_address();
        assert_ne!(data1, data2);
    }

    #[test]
    fn username() {
        let data1 = internet::username();
        let data2 = internet::username();
        assert_ne!(data1, data2);
    }
}