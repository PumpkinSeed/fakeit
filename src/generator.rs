use crate::contact;
use crate::data::person;
use crate::hacker;

use crate::misc;

pub fn generate(data: String) -> String {
    let mut d_validate_left = data.matches("{").count();
    let mut d_validate_right = data.matches("}").count();

    let mut m_data = data.clone();

    while d_validate_left > 0 && d_validate_right > 0 && d_validate_left == d_validate_right {
        let place_of_left = m_data.find('{').unwrap();
        let place_of_right = m_data.find('}').unwrap();

        let tag = &m_data[place_of_left + 1..place_of_right];
        let text = resolve_tag(tag);
        let tag_formatted = format!("{}{}{}", "{", tag, "}");
        m_data = m_data.replace(&tag_formatted[..], &text[..]);

        d_validate_left = m_data.matches("{").count();
        d_validate_right = m_data.matches("}").count();
    }

    m_data = misc::replace_with_numbers(m_data);
    m_data = misc::replace_with_letter(m_data);

    m_data
}

fn resolve_tag(tag: &str) -> String {
    match tag {
        "contact.email" => return contact::email(),
        "hacker.abbreviation" => return hacker::abbreviation(),
        "hacker.adjective" => return hacker::adjective(),
        "hacker.noun" => return hacker::noun(),
        "hacker.verb" => return hacker::verb(),
        "hacker.ingverb" => return hacker::ingverb(),
        "person.first" => return misc::random_data_str(person::FIRST).to_string(),
        "person.last" => return misc::random_data_str(person::LAST).to_string(),

        _ => return "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::generator;

    #[test]
    fn generate() {
        let new =
            generator::generate("{person.first} {person.last} {contact.email} #?#?#?".to_string());
        println!("{}", new);
    }
}

// func Generate(dataVal string) string {
// 	// Identify items between brackets: {person.first}
// 	for strings.Count(dataVal, "{") > 0 && strings.Count(dataVal, "}") > 0 {
// 		catValue := ""
// 		startIndex := strings.Index(dataVal, "{")
// 		endIndex := strings.Index(dataVal, "}")
// 		replace := dataVal[(startIndex + 1):endIndex]
// 		categories := strings.Split(replace, ".")

// 		if len(categories) >= 2 && dataCheck([]string{categories[0], categories[1]}) {
// 			catValue = getRandValue([]string{categories[0], categories[1]})
// 		}

// 		dataVal = strings.Replace(dataVal, "{"+replace+"}", catValue, 1)
// 	}

// 	// Replace # with numbers
// 	dataVal = replaceWithNumbers(dataVal)

// 	// Replace ? with letters
// 	dataVal = replaceWithLetters(dataVal)

// 	return dataVal
// }
