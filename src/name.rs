use crate::data::person;
use crate::misc;
use ::std::string::String;

pub fn full() -> String {
    format!("{} {}", first(), last())
}

pub fn first() -> String {
    misc::random_data(person::FIRST).to_string()
}

pub fn last() -> String {
    misc::random_data(person::LAST).to_string()
}

pub fn prefix() -> String {
    misc::random_data(person::PREFIX).to_string()
}

pub fn suffix() -> String {
    misc::random_data(person::SUFFIX).to_string()
}

#[cfg(test)]
mod tests {
    use crate::name;
    use crate::testify::exec_mes;

    #[test]
    fn full() {
        exec_mes("name::full", name::full);
    }

    #[test]
    fn first() {
        exec_mes("name::first", name::first);
    }

    #[test]
    fn last() {
        exec_mes("name::last", name::last);
    }

    #[test]
    fn prefix() {
        exec_mes("name::prefix", name::prefix);
    }

    #[test]
    fn suffix() {
        exec_mes("name::suffix", name::suffix);
    }
}
