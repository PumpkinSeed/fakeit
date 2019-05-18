use crate::data::files;
use crate::misc;

pub fn mime_type() -> String {
    misc::random_data_str(files::MIME_TYPE).to_string()
}

pub fn extension() -> String {
    misc::random_data_str(files::EXTENSION).to_string()
}
