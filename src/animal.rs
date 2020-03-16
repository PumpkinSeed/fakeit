use crate::data::animal;
use crate::misc;

pub fn pet_name() -> String {
    misc::random_data(animal::PETNAME).to_string()
}

pub fn animal() -> String {
    misc::random_data(animal::ANIMAL).to_string()
}

pub fn type_of() -> String {
    misc::random_data(animal::TYPE).to_string()
}

pub fn farm() -> String {
    misc::random_data(animal::FARM).to_string()
}

pub fn cat() -> String {
    misc::random_data(animal::CAT).to_string()
}

pub fn dog() -> String {
    misc::random_data(animal::DOG).to_string()
}

#[cfg(test)]
mod tests {
    use crate::animal;
    use crate::test_helper;

    #[test]
    fn pet_name() {
        let data1 = animal::pet_name();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn animal() {
        let data1 = animal::animal();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn type_of() {
        let data1 = animal::type_of();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn farm() {
        let data1 = animal::farm();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn cat() {
        let data1 = animal::cat();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn dog() {
        let data1 = animal::dog();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    use std::time::{Instant};
    #[test]
    fn dog_bench() {
        let start = Instant::now();
        animal::dog();
        let duration = start.elapsed();

        println!("Time elapsed in animal::dog() is: {:?}", duration);
    }
}