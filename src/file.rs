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
    use crate::testify::exec_mes;

    #[test]
    fn mime_type() {
        exec_mes("file::mime_type", file::mime_type);
    }

    #[test]
    fn extension() {
        exec_mes("file::extension", file::extension);
    }
}
