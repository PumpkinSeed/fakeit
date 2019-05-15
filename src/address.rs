use ::std::string::String;

pub struct AddressInfo {
    address: String,
    street: String,
    city: String,
    state: String,
    zip: String,
    country: String,
    latitude: f64,
    longitude: f64,
}

pub fn address() -> AddressInfo {
    AddressInfo {
        address: "".to_string(),
        street: "".to_string(),
        city: "".to_string(),
        state: "".to_string(),
        zip: "".to_string(),
        country: "".to_string(),
        latitude: 0.0,
        longitude: 0.0,
    }
}

#[cfg(test)]
mod tests {
    use crate::data::address;

    #[test]
    fn street_name() {
        let street = address::STREET_NAME[2];
        assert_eq!(street, "Branch");
    }
}
