use crate::data::log_level;
use crate::misc;

pub fn general() -> String {
    misc::random_data(log_level::GENERAL).to_string()
}

pub fn syslog() -> String {
    misc::random_data(log_level::SYSLOG).to_string()
}

pub fn apache() -> String {
    misc::random_data(log_level::APACHE).to_string()
}

#[cfg(test)]
mod tests {
    use crate::log_level;
    use crate::test_helper;

    #[test]
    fn general() {
        let data1 = log_level::general();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn apache() {
        let data1 = log_level::apache();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn syslog() {
        let data1 = log_level::syslog();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}