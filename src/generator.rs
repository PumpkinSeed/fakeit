use simplerand::{Randomable, Rng};
use crate::contact;
use crate::data::person;
use crate::hacker;
use crate::data::address;

use crate::misc;

pub const HASHTAG: &str = "#";

pub struct Generator {
    pub rng: Rng,
}

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
        "person.first" => return misc::random_data(person::FIRST).to_string(),
        "person.last" => return misc::random_data(person::LAST).to_string(),

        _ => return "".to_string(),
    }
}

impl Generator {
    pub fn new(rng: Rng) -> Generator {
        Generator { rng }
    }

    pub fn replace_with_numbers(&mut self, s: String) -> String {
        if s == String::from("") {
            return s;
        }

        let res: Vec<String> = s
            .split("")
            .map(|s| {
                if s == HASHTAG {
                    let i = self.random::<i64>(0, 9);
                    return i.to_string();
                }
                s.to_string()
            })
            .collect();

        res.join("")
    }

    pub fn random<T: Randomable>(&mut self, min: T, max: T) -> T {
        self.rng.rand_range(min, max)
    }

    pub fn random_data<T: Clone>(&mut self, d: &[T]) -> T {
        let n = rand_range(0, d.len() as i64);
        let res = d[n as usize].clone();
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::generator;
    use crate::testify::exec_mes;

    #[test]
    fn generate() {
        exec_mes("beer::name", || {
            generator::generate("{person.first} {person.last} {contact.email} #?#?#?".to_string())
        });
    }
}
