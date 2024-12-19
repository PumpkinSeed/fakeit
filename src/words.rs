use crate::data::lorem;
use crate::hipster;
use crate::misc;
use crate::name;

pub struct ParagraphOpts {
    count: i64,
    sentence_count: i64,
    word_count: i64,
    separator: String,
}

pub fn word() -> String {
    misc::random_data(lorem::WORD).to_string()
}

pub fn sentence(word_count: i64) -> String {
    if word_count <= 0 {
        return "".to_string();
    }

    let mut sentence_vec = Vec::<String>::new();
    for i in 0..word_count {
        if i == 0 {
            sentence_vec.push(title(word()))
        } else if i == word_count - 1 {
            let word_with_dot = format!("{}.", word());
            sentence_vec.push(word_with_dot)
        } else {
            sentence_vec.push(word())
        }
    }

    sentence_vec.join(" ")
}

pub fn paragraph(count: i64, sentence_count: i64, word_count: i64, separator: String) -> String {
    let opts = ParagraphOpts {
        count,
        sentence_count,
        word_count,
        separator,
    };

    paragraph_generator(opts, &sentence)
}

pub fn paragraph_generator(
    opts: ParagraphOpts,
    sentence_generator: &dyn Fn(i64) -> String,
) -> String {
    let mut paragraph_vec = Vec::<String>::new();
    for _i in 0..opts.count {
        let mut sentence_vec = Vec::<String>::new();
        for _i in 0..opts.sentence_count {
            sentence_vec.push(sentence_generator(opts.word_count));
        }
        paragraph_vec.push(sentence_vec.join(" "))
    }
    paragraph_vec.join(&opts.separator[..])
}

pub fn question() -> String {
    hipster::sentence(misc::random(3, 10)).replace(".", "?")
}

pub fn quote() -> String {
    format!(
        "\"{}\" - {} {}",
        hipster::sentence(misc::random(3, 10)),
        name::first(),
        name::last()
    )
}

fn title(s: String) -> String {
    let mut v: Vec<char> = s.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    v.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::testify::exec_mes;
    use crate::words;

    #[test]
    fn word() {
        exec_mes("words::word", || words::word());
    }

    #[test]
    fn sentence() {
        exec_mes("words::sentence", || words::sentence(10));
    }

    #[test]
    fn paragraph() {
        exec_mes("words::paragraph", || {
            words::paragraph(5, 4, 11, "\n".to_string())
        });
    }

    #[test]
    fn question() {
        exec_mes("words::question", || words::question());
    }

    #[test]
    fn quote() {
        exec_mes("words::quote", || words::quote());
    }

    #[test]
    fn paragraph_generator() {
        exec_mes("words::paragraph_generator", || {
            let opts = words::ParagraphOpts {
                count: 5,
                sentence_count: 4,
                word_count: 11,
                separator: "\n".to_string(),
            };
            words::paragraph_generator(opts, &words::sentence)
        });
    }
}
