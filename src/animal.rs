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
    use crate::testify::exec_mes;

    #[test]
    fn pet_name() {
        exec_mes("animal::pet_name", animal::pet_name);
    }

    #[test]
    fn animal() {
        exec_mes("animal::animal", animal::animal);
    }

    #[test]
    fn type_of() {
        exec_mes("animal::type_of", animal::type_of);
    }

    #[test]
    fn farm() {
        exec_mes("animal::farm", animal::farm);
    }

    #[test]
    fn cat() {
        exec_mes("animal::cat", animal::cat);
    }

    #[test]
    fn dog() {
        exec_mes("animal::dog", animal::dog);
    }
}
