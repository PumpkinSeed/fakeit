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
    use crate::testify::exec_mes;

    #[test]
    fn vehicle_type() {
        exec_mes("vehicle::vehicle_type", || {
            vehicle::vehicle_type()
        });
    }

    #[test]
    fn fuel() {
        exec_mes("vehicle::fuel", || {
            vehicle::fuel()
        });
    }

    #[test]
    fn transmission_gear() {
        exec_mes("vehicle::transmission_gear", || {
            vehicle::transmission_gear()
        });
    }

    #[test]
    fn car_maker() {
        exec_mes("vehicle::car_maker", || {
            vehicle::car_maker()
        });
    }

    #[test]
    fn car_model() {
        exec_mes("vehicle::car_model", || {
            vehicle::car_model()
        });
    }
}
