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

    #[test]
    fn pet_name() {
        let pet_name1 = animal::pet_name();
        let pet_name2 = animal::pet_name();
        assert_ne!(pet_name1, pet_name2);
    }

    #[test]
    fn animal() {
        let animal1 = animal::animal();
        let animal2 = animal::animal();
        assert_ne!(animal1, animal2);
    }

    #[test]
    fn type_of() {
        let type_of1 = animal::type_of();
        let type_of2 = animal::type_of();
        assert_ne!(type_of1, type_of2);
    }

    #[test]
    fn farm() {
        let farm1 = animal::farm();
        let farm2 = animal::farm();
        assert_ne!(farm1, farm2);
    }

    #[test]
    fn cat() {
        let cat1 = animal::cat();
        let cat2 = animal::cat();
        assert_ne!(cat1, cat2);
    }

    #[test]
    fn dog() {
        let dog1 = animal::dog();
        let dog2 = animal::dog();
        assert_ne!(dog1, dog2);
    }
}