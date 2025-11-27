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
        misc::random_data(person::LAST),
        misc::replace_with_numbers("####".to_string()),
    )
}

#[cfg(test)]
mod tests {
    use crate::internet;
    use crate::testify::exec_mes;

    #[test]
    fn domain_name() {
        exec_mes("internet::domain_name", internet::domain_name);
    }

    #[test]
    fn http_method() {
        exec_mes("internet::http_method", internet::http_method);
    }

    #[test]
    fn domain_suffix() {
        exec_mes("internet::domain_suffix", internet::domain_suffix);
    }

    #[test]
    fn ipv4_address() {
        exec_mes("internet::ipv4_address", internet::ipv4_address);

        let data1 = internet::ipv4_address();
        let data2 = internet::ipv4_address();
        assert_ne!(data1, data2);
    }

    #[test]
    fn ipv6_address() {
        exec_mes("internet::ipv6_address", internet::ipv6_address);

        let data1 = internet::ipv6_address();
        let data2 = internet::ipv6_address();
        assert_ne!(data1, data2);
    }

    #[test]
    fn mac_address() {
        exec_mes("internet::mac_address", internet::mac_address);

        let data1 = internet::mac_address();
        let data2 = internet::mac_address();
        assert_ne!(data1, data2);
    }

    #[test]
    fn username() {
        exec_mes("internet::username", internet::username);
    }
}
