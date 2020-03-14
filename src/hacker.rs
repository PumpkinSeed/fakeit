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

    #[test]
    fn phrase() {
        let new = hacker::phrase();
        println!("{}", new);
    }
}
