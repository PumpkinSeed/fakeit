use crate::data::language;
use crate::misc;

pub fn random() -> String {
    misc::random_data(language::LONG).to_string()
}

pub fn abbreviation() -> String {
    misc::random_data(language::SHORT).to_string()
}

pub fn programming() -> String {
    misc::random_data(language::PROGRAMMING).to_string()
}

#[cfg(test)]
mod tests {
    use crate::language;
    use crate::testify::exec_mes;

    #[test]
    fn random() {
        exec_mes("language::random", language::random);
    }

    #[test]
    fn abbreviation() {
        exec_mes("language::abbreviation", language::abbreviation);
    }

    #[test]
    fn programming() {
        exec_mes("language::programming", language::programming);
    }
}
