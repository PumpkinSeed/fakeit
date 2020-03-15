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
