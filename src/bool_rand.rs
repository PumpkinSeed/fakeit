use crate::misc;

pub fn bool() -> bool {
    if misc::random_int(0, 1) == 1 {
        return true;
    }

    false
}
