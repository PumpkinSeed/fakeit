use crate::data::color;
use crate::misc;

pub fn full() -> String {
    misc::random_data_str(color::FULL).to_string()
}

pub fn hex() -> String {
    let mut rand: [&'static str; 6] = [
        misc::HASHTAG,
        misc::HASHTAG,
        misc::HASHTAG,
        misc::HASHTAG,
        misc::HASHTAG,
        misc::HASHTAG,
    ];

    for x in 0..5 {
        match misc::random_int16(0, 1) {
            0 => rand[x] = misc::HASHTAG,
            1 => rand[x] = misc::QUESTIONMARK,
            _ => println!("impossible"),
        }
    }

    format!(
        "#{}",
        misc::replace_with_letter_hex(misc::replace_with_numbers(rand.join("")))
    )
}

pub fn safe() -> String {
    misc::random_data_str(color::SAFE).to_string()
}

pub fn rgb() -> [i16; 3] {
    [
        misc::random_int16(0, 255),
        misc::random_int16(0, 255),
        misc::random_int16(0, 255),
    ]
}
