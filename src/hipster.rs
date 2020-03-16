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
    use crate::test_helper;

    #[test]
    fn word() {
        let data1 = hipster::word();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn sentence() {
        let data1 = hipster::sentence(12);
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn paragraph() {
        let data1 = hipster::paragraph(3, 4, 40, " ".to_string());
        let data2 = hipster::paragraph(3, 4, 40, " ".to_string());
        assert_ne!(data1, data2);
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}