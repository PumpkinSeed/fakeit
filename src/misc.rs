extern crate rand;

use rand::{thread_rng, Rng};

pub fn random_data_str(d: &'static [&'static str]) -> &'static str {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(0, d.len());

    d[n]
}

#[cfg(test)]
mod tests {
    use crate::data::address;
    use crate::misc;

    #[test]
    fn random_data_str() {
        let street1 = misc::random_data_str(address::STREET_NAME);
        let street2 = misc::random_data_str(address::STREET_NAME);
        assert_ne!(street1, street2);
    }
}
