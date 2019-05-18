use crate::data::hacker;
use crate::misc;

pub fn phrase() -> String {
    misc::random_data_str(hacker::PHRASE).to_string()
    // @TODO
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

// // HackerPhrase will return a random hacker sentence
// func HackerPhrase() string {
// 	words := strings.Split(Generate(getRandValue([]string{"hacker", "phrase"})), " ")
// 	words[0] = strings.Title(words[0])
// 	return strings.Join(words, " ")
// }
