use crate::data::lorem;
use crate::misc;

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
        count: count,
        sentence_count: sentence_count,
        word_count: word_count,
        separator: separator,
    };

    paragraph_generator(opts, &sentence)
}

pub fn paragraph_generator(opts: ParagraphOpts, sentence_generator: &Fn(i64) -> String) -> String {
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

fn title(s: String) -> String {
    let mut v: Vec<char> = s.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    v.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::words;

    #[test]
    fn phrase() {
        let new = words::sentence(100);
        println!("{}", new);
    }

    #[test]
    fn paragraph() {
        let opts = words::ParagraphOpts {
            count: 5,
            sentence_count: 4,
            word_count: 11,
            separator: "\n".to_string(),
        };
        let new = words::paragraph_generator(opts, &words::sentence);
        println!("{}", new);
    }
}

// // Question will return a random question
// func Question() string {
// 	return strings.Replace(HipsterSentence(Number(3, 10)), ".", "?", 1)
// }

// // Quote will return a random quote from a random person
// func Quote() string {
// 	return `"` + HipsterSentence(Number(3, 10)) + `" - ` + FirstName() + " " + LastName()
// }
