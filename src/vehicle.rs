extern crate chrono;

use crate::data::vehicle;
use crate::misc;
use chrono::{Datelike, Utc};

pub struct Info {
    vehicle_type: String,
    fuel: String,
    transmission_gear: String,
    brand: String,
    model: String,
    year: i32,
}

pub fn info() -> Info {
    Info {
        vehicle_type: vehicle_type(),
        fuel: fuel(),
        transmission_gear: transmission_gear(),
        brand: car_maker(),
        model: car_model(),
        year: misc::random::<i32>(0, Utc::now().year()),
    }
}

pub fn vehicle_type() -> String {
    misc::random_data(vehicle::TYPE).to_string()
}

pub fn fuel() -> String {
    misc::random_data(vehicle::FUEL_TYPE).to_string()
}

pub fn transmission_gear() -> String {
    misc::random_data(vehicle::TRANSMISSION_TYPE).to_string()
}

pub fn car_maker() -> String {
    misc::random_data(vehicle::MAKER).to_string()
}

pub fn car_model() -> String {
    misc::random_data(vehicle::MODEL).to_string()
}


#[cfg(test)]
mod tests {
    use crate::vehicle;
    use crate::test_helper;

    #[test]
    fn vehicle_type() {
        let data1 = vehicle::vehicle_type();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn fuel() {
        let data1 = vehicle::fuel();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn transmission_gear() {
        let data1 = vehicle::transmission_gear();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn car_maker() {
        let data1 = vehicle::car_maker();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn car_model() {
        let data1 = vehicle::car_model();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }
}
