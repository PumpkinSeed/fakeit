use crate::data::language;
use crate::misc;

pub fn random() -> String {
    misc::random_data(language::LONG).to_string()
}

pub fn abbreviation() -> String {
    misc::random_data(language::SHORT).to_string()
}

pub fn programming() -> String {
    misc::random_data(language::PROGRAMMING).to_string()
}

#[cfg(test)]
mod tests {
    use crate::language;
    use crate::test_helper;

    #[test]
    fn random() {
        let data1 = language::random();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn abbreviation() {
        let data1 = language::abbreviation();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn programming() {
        let data1 = language::programming();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}