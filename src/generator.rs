use crate::data::hacker;
use crate::misc;

pub fn generate(data: String) -> String {
    let d_validate_left = data.matches("{").count();
    let d_validate_right = data.matches("}").count();

    let mut m_data = data.clone();

    if d_validate_left > 0 && d_validate_right > 0 && d_validate_left == d_validate_right {
        let cat_value = "".to_string();
        let place_of_left = m_data.find('{').unwrap();
        let place_of_right = m_data.find('}').unwrap();
        println!("{}{}", place_of_left, place_of_right);

        let tag = &m_data[place_of_left + 1..place_of_right];
        let text = resolve_tag(tag);
        let tag_formatted = format!("{}{}{}", "{", tag, "}");
        m_data = m_data.replace("{person.first}", text);
    }

    m_data = misc::replace_with_numbers(m_data);
    m_data = misc::replace_with_letter(m_data);

    m_data
}

fn resolve_tag(tag: &str) -> &str {
    "tehen"
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
