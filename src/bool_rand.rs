use crate::misc;

pub fn bool() -> bool {
    if misc::random::<i64>(0, 1) == 1 {
        return true;
    }

    false
}
