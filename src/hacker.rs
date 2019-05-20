use crate::data::hacker;
use crate::generator;
use crate::misc;

pub fn phrase() -> String {
    let phrase = misc::random_data_str(hacker::PHRASE).to_string();
    generator::generate(phrase)
}

pub fn abbreviation() -> String {
    misc::random_data_str(hacker::ABBREVIATION).to_string()
}

pub fn adjective() -> String {
    misc::random_data_str(hacker::ADJECTIVE).to_string()
}

pub fn noun() -> String {
    misc::random_data_str(hacker::NOUN).to_string()
}

pub fn verb() -> String {
    misc::random_data_str(hacker::VERB).to_string()
}

pub fn ingverb() -> String {
    misc::random_data_str(hacker::INGVERB).to_string()
}

#[cfg(test)]
mod tests {
    use crate::hacker;

    #[test]
    fn phrase() {
        let new = hacker::phrase();
        println!("{}", new);
    }
}
