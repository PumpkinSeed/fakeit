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
    use crate::testify::exec_mes;

    #[test]
    fn phrase() {
        exec_mes("hacker::phrase", hacker::phrase);
    }

    #[test]
    fn abbreviation() {
        exec_mes("hacker::abbreviation", hacker::abbreviation);
    }

    #[test]
    fn adjective() {
        exec_mes("hacker::adjective", hacker::adjective);
    }

    #[test]
    fn noun() {
        exec_mes("hacker::noun", hacker::noun);
    }

    #[test]
    fn verb() {
        exec_mes("hacker::verb", hacker::verb);
    }

    #[test]
    fn ingverb() {
        exec_mes("hacker::ingverb", hacker::ingverb);
    }
}
