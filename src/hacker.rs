use crate::data::hacker;
use crate::generator;
use crate::misc;

pub fn phrase() -> String {
    let phrase = misc::random_data(hacker::PHRASE).to_string();
    generator::generate(phrase)
}

pub fn abbreviation() -> String {
    misc::random_data(hacker::ABBREVIATION).to_string()
}

pub fn adjective() -> String {
    misc::random_data(hacker::ADJECTIVE).to_string()
}

pub fn noun() -> String {
    misc::random_data(hacker::NOUN).to_string()
}

pub fn verb() -> String {
    misc::random_data(hacker::VERB).to_string()
}

pub fn ingverb() -> String {
    misc::random_data(hacker::INGVERB).to_string()
}

#[cfg(test)]
mod tests {
    use crate::hacker;
    use crate::test_helper;

    #[test]
    fn phrase() {
        let data1 = hacker::phrase();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn abbreviation() {
        let data1 = hacker::abbreviation();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn adjective() {
        let data1 = hacker::adjective();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn noun() {
        let data1 = hacker::noun();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn verb() {
        let data1 = hacker::verb();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn ingverb() {
        let data1 = hacker::ingverb();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}
