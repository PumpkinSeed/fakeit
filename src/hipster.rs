use crate::data::hipster;
use crate::misc;
use crate::words;

pub fn word() -> String {
    misc::random_data_str(hipster::WORD).to_string()
}

pub fn sentence(word_count: i64) -> String {
    words::sentence(word_count)
}

pub fn paragraph(count: i64, sentence_count: i64, word_count: i64, separator: String) -> String {
    words::paragraph(count, sentence_count, word_count, separator)
}
