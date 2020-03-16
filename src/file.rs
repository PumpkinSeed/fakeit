use crate::data::files;
use crate::misc;

pub fn mime_type() -> String {
    misc::random_data(files::MIME_TYPE).to_string()
}

pub fn extension() -> String {
    misc::random_data(files::EXTENSION).to_string()
}

#[cfg(test)]
mod tests {
    use crate::file;
    use crate::test_helper;

    #[test]
    fn mime_type() {
        let data1 = file::mime_type();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }

    }

    #[test]
    fn extension() {
        let data1 = file::extension();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}