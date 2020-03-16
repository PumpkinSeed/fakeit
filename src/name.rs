use crate::data::person;
use crate::misc;
use ::std::string::String;

pub fn full() -> String {
    format!("{} {}", first(), last())
}

pub fn first() -> String {
    misc::random_data(person::FIRST).to_string()
}

pub fn last() -> String {
    misc::random_data(person::LAST).to_string()
}

pub fn prefix() -> String {
    misc::random_data(person::PREFIX).to_string()
}

pub fn suffix() -> String {
    misc::random_data(person::SUFFIX).to_string()
}

#[cfg(test)]
mod tests {
    use crate::name;
    use crate::test_helper;

    #[test]
    fn full() {
        let data1 = name::full();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn first() {
        let data1 = name::first();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn last() {
        let data1 = name::last();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn prefix() {
        let data1 = name::prefix();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn suffix() {
        let data1 = name::suffix();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}