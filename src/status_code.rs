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
    use crate::test_helper;

    #[test]
    fn simple() {
        let data1 = status_code::simple();
        assert_ne!(data1, 0 as i16);
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn general() {
        let data1 = status_code::general();
        assert_ne!(data1, 0 as i16);
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}