use crate::company;
use crate::data::job;
use crate::misc;

pub struct Info {
    company: String,
    title: String,
    descriptor: String,
    level: String,
}

pub fn info() -> Info {
    Info {
        company: company::company(),
        title: title(),
        descriptor: descriptor(),
        level: level(),
    }
}

pub fn title() -> String {
    misc::random_data(job::TITLE).to_string()
}

pub fn descriptor() -> String {
    misc::random_data(job::DESCRIPTOR).to_string()
}

pub fn level() -> String {
    misc::random_data(job::LEVEL).to_string()
}

#[cfg(test)]
mod tests {
    use crate::job;
    use crate::testify::exec_mes;

    #[test]
    fn title() {
        exec_mes("job::title", job::title);
    }

    #[test]
    fn descriptor() {
        exec_mes("job::descriptor", job::descriptor);
    }

    #[test]
    fn level() {
        exec_mes("job::level", job::level);
    }
}
