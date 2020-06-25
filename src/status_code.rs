use crate::data::status_code;
use crate::misc;

pub fn simple() -> i16 {
    misc::random_data(status_code::SIMPLE)
}

pub fn general() -> i16 {
    misc::random_data(status_code::GENERAL)
}

#[cfg(test)]
mod tests {
    use crate::status_code;
    use crate::testify::exec_mes;

    #[test]
    fn simple() {
        exec_mes("status_code::simple", || {
            format!("{}", status_code::simple())
        });
    }

    #[test]
    fn general() {
        exec_mes("status_code::general", || {
            format!("{}", status_code::general())
        });
    }
}
