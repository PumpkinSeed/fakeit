#[allow(dead_code)]
pub static CARD_TYPE: &'static [&str] = &["Visa", "MasterCard", "American Express", "Discover"];

#[allow(dead_code)]
pub static NUMBER: &'static [&str] = &[
    // Visa
    "4###############",
    "4###############",
    // Mastercard
    "222100##########",
    "272099##########",
    // American Express
    "34#############",
    "37#############",
    // Discover
    "65##############",
    "65##############",
];
