use crate::data::lorem;
use crate::misc;

pub struct ParagraphOpts {
    count: i64,
    sentence_count: i64,
    word_count: i64,
    separator: String,
}

pub fn word() -> String {
    misc::random_data_str(lorem::WORD).to_string()
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
    "".to_string()
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
        let new = words::sentence(10);
        println!("{}", new);
    }
}

// const bytesPerWordEstimation = 6

// type sentenceGenerator func(wordCount int) string
// type wordGenerator func() string

// // Word will generate a random word
// func Word() string {
// 	return getRandValue([]string{"lorem", "word"})
// }

// // Sentence will generate a random sentence
// func Sentence(wordCount int) string {
// 	return sentence(wordCount, Word)
// }

// // Paragraph will generate a random paragraphGenerator
// // Set Paragraph Count
// // Set Sentence Count
// // Set Word Count
// // Set Paragraph Separator
// func Paragraph(paragraphCount int, sentenceCount int, wordCount int, separator string) string {
// 	return paragraphGenerator(paragrapOptions{paragraphCount, sentenceCount, wordCount, separator}, Sentence)
// }

// func paragraphGenerator(opts paragrapOptions, sentecer sentenceGenerator) string {
// 	if opts.paragraphCount <= 0 || opts.sentenceCount <= 0 || opts.wordCount <= 0 {
// 		return ""
// 	}

// 	//to avoid making Go 1.10 dependency, we cannot use strings.Builder
// 	paragraphs := bytes.Buffer{}
// 	//we presume the length
// 	paragraphs.Grow(opts.paragraphCount * opts.sentenceCount * opts.wordCount * bytesPerWordEstimation)
// 	wordSeparator := ' '

// 	for i := 0; i < opts.paragraphCount; i++ {
// 		for e := 0; e < opts.sentenceCount; e++ {
// 			paragraphs.WriteString(sentecer(opts.wordCount))
// 			if e < opts.sentenceCount-1 {
// 				paragraphs.WriteRune(wordSeparator)
// 			}
// 		}

// 		if i < opts.paragraphCount-1 {
// 			paragraphs.WriteString(opts.separator)
// 		}
// 	}

// 	return paragraphs.String()
// }

// // Question will return a random question
// func Question() string {
// 	return strings.Replace(HipsterSentence(Number(3, 10)), ".", "?", 1)
// }

// // Quote will return a random quote from a random person
// func Quote() string {
// 	return `"` + HipsterSentence(Number(3, 10)) + `" - ` + FirstName() + " " + LastName()
// }
