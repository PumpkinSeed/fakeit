extern crate uuid;

use std::time::{SystemTime, UNIX_EPOCH};
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

pub fn uuid_v1() -> String {
    let context = Context::new(42);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before Unix epoch");
    let ts = Timestamp::from_unix(&context, now.as_secs(), now.subsec_nanos());
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
