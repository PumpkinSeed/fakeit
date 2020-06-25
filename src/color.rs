use crate::data::color;
use crate::misc;

pub fn full() -> String {
    misc::random_data(color::FULL).to_string()
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
        match misc::random::<i8>(0, 1) {
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
    misc::random_data(color::SAFE).to_string()
}

pub fn rgb() -> [i16; 3] {
    [
        misc::random::<i16>(0, 255),
        misc::random::<i16>(0, 255),
        misc::random::<i16>(0, 255),
    ]
}

#[cfg(test)]
mod tests {
    use crate::color;
    use crate::testify::exec_mes;

    #[test]
    fn full() {
        exec_mes("color::full", || color::full());
    }

    #[test]
    fn hex() {
        exec_mes("color::hex", || color::hex());
    }

    #[test]
    fn safe() {
        exec_mes("color::safe", || color::safe());
    }

    #[test]
    fn rgb() {
        exec_mes("color::rgb", || format!("{:?}", color::rgb()));
    }
}
