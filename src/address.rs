#[cfg(test)]
mod tests {
    use crate::address_data;

    #[test]
    fn street_name() {
        let street = address_data::STREET_NAME[2];
        assert_eq!(street, "Branch");
    }
}
