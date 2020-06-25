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
    use crate::testify::exec_mes;
    use crate::unique;

    #[test]
    fn uuid_v1() {
        exec_mes("unique::uuid_v1", || unique::uuid_v1());
    }

    #[test]
    fn uuid_v4() {
        exec_mes("unique::uuid_v4", || unique::uuid_v4());
    }
}
