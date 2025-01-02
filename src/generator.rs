use crate::contact;
use crate::data::person;
use crate::hacker;
use lazy_static::lazy_static;
use simplerand::{base, Random};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::misc;

pub const HASHTAG: &str = "#";

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
        "contact.email" => contact::email(),
        "hacker.abbreviation" => hacker::abbreviation(),
        "hacker.adjective" => hacker::adjective(),
        "hacker.noun" => hacker::noun(),
        "hacker.verb" => hacker::verb(),
        "hacker.ingverb" => hacker::ingverb(),
        "person.first" => misc::random_data(person::FIRST).to_string(),
        "person.last" => misc::random_data(person::LAST).to_string(),

        _ => "".to_string(),
    }
}

lazy_static! {
    pub static ref BASE_GENERATOR: Generator = Generator::new_default();
}

pub struct Generator {
    pub rng: Random,
}

impl Generator {
    pub fn new(seed: u128) -> Generator {
        Generator {
            rng: Random::new(seed),
        }
    }

    pub fn new_default() -> Generator {
        Generator {
            rng: Random::new(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            ),
        }
    }

    pub fn replace_with_numbers(&self, s: String) -> String {
        if s == *"" {
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

    pub fn random<T: base::Randomable>(&self, min: T, max: T) -> T {
        self.rng.rand_range(min, max)
    }

    pub fn random_data<T: Clone>(&self, d: &[T]) -> T {
        let n = self.rng.rand_range(0, d.len() as u128);
        d[n as usize].clone()
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
