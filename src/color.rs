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
    use crate::test_helper;

    #[test]
    fn full() {
        let data1 = color::full();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn hex() {
        let data1 = color::hex();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn safe() {
        let data1 = color::safe();
        assert_ne!(data1, "");
        if test_helper::print() {
            println!("{}", data1);
        }
    }

    #[test]
    fn rgb() {
        let data1 = color::rgb();
        assert_eq!(data1.len(), 3);
        if test_helper::print() {
            println!("{}, {}, {}", data1[0], data1[1], data1[2]);
        }
    }
}
