extern crate chrono;
extern crate uuid;

use chrono::{Timelike, Utc};
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

pub fn uuid_v1() -> String {
    let context = Context::new(42);
    let ts = Timestamp::from_unix(
        &context,
        Utc::now().second() as u64,
        Utc::now().timestamp_subsec_nanos(),
    );
    let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6]).expect("failed to generate UUID");
    uuid.to_string()
}

pub fn uuid_v4() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use crate::unique;
    use crate::test_helper;

    #[test]
    fn uuid_v1() {
        let data1 = unique::uuid_v1();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn uuid_v4() {
        let data1 = unique::uuid_v4();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}
