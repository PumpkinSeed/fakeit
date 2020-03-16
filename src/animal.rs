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
        let pet_name = animal::pet_name();
        println!("pet_name: {}", pet_name);
    }

    #[test]
    fn animal() {
        let animal = animal::animal();
        println!("animal: {}", animal);
    }

    #[test]
    fn type_of() {
        let type_of = animal::type_of();
        println!("type_of: {}", type_of);
    }

    #[test]
    fn farm() {
        let farm = animal::farm();
        println!("farm: {}", farm);
    }

    #[test]
    fn cat() {
        let cat = animal::cat();
        println!("cat: {}", cat);
    }

    #[test]
    fn dog() {
        let dog = animal::dog();
        println!("dog: {}", dog);
    }
}