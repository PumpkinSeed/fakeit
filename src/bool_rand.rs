use crate::misc;

pub fn bool() -> bool {
    misc::random::<i64>(0, 1) == 1
}
