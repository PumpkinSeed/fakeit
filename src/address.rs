#[cfg(test)]
mod tests {
    use crate::data::address;

    #[test]
    fn street_name() {
        let street = address::STREET_NAME[2];
        assert_eq!(street, "Branch");
    }
}
