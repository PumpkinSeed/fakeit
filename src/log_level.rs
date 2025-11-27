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
    use crate::testify::exec_mes;

    #[test]
    fn general() {
        exec_mes("log_level::general", log_level::general);
    }

    #[test]
    fn apache() {
        exec_mes("log_level::apache", log_level::apache);
    }

    #[test]
    fn syslog() {
        exec_mes("log_level::syslog", log_level::syslog);
    }
}
