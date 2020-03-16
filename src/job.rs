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
    use crate::test_helper;

    #[test]
    fn title() {
        let data1 = job::title();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn descriptor() {
        let data1 = job::descriptor();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn level() {
        let data1 = job::level();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}