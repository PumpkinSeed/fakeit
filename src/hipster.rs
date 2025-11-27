use crate::data::hipster;
use crate::misc;
use crate::words;

pub fn word() -> String {
    misc::random_data(hipster::WORD).to_string()
}

pub fn sentence(word_count: i64) -> String {
    words::sentence(word_count)
}

pub fn paragraph(count: i64, sentence_count: i64, word_count: i64, separator: String) -> String {
    words::paragraph(count, sentence_count, word_count, separator)
}

#[cfg(test)]
mod tests {
    use crate::hipster;
    use crate::testify::exec_mes;

    #[test]
    fn word() {
        exec_mes("hipster::word", hipster::word);
    }

    #[test]
    fn sentence() {
        exec_mes("hipster::sentence", || hipster::sentence(12));
    }

    #[test]
    fn paragraph() {
        let data1 = hipster::paragraph(3, 4, 40, " ".to_string());
        let data2 = hipster::paragraph(3, 4, 40, " ".to_string());
        assert_ne!(data1, data2);
    }
}
