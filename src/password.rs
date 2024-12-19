use crate::misc;

pub fn generate(upper: bool, numeric: bool, special: bool, mut num: i8) -> String {
    if num < 5 {
        num = 5;
    }
    letter(upper, numeric, special, num)
}

fn letter(upper: bool, numeric: bool, special: bool, num: i8) -> String {
    let mut pw = String::from("");

    let mut opts = vec![1i8];
    if upper {
        opts.push(2);
    }
    if numeric {
        opts.push(3);
    }
    if special {
        opts.push(4);
    }

    let lower_str = b"abcdefghijklmnopqrstuvwxyz";
    let upper_str = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numeric_str = b"0123456789";
    let special_str = b"!@#$%&*+-=?";

    match opts[misc::random_data_index(opts.as_slice())] {
        1 => pw.push(misc::random_char_from_string(lower_str)),
        2 => pw.push(misc::random_char_from_string(upper_str)),
        3 => pw.push(misc::random_char_from_string(numeric_str)),
        4 => pw.push(misc::random_char_from_string(special_str)),
        _ => pw.push(misc::random_char_from_string(lower_str)),
    }

    match num {
        0 => pw,
        _ => format!("{}{}", pw, letter(upper, numeric, special, num - 1)),
    }
}

#[cfg(test)]
mod tests {
    use crate::password;
    use crate::testify::exec_mes;

    #[test]
    fn password_generate() {
        exec_mes("password::generate", || {
            password::generate(true, true, true, 26)
        });
    }
}
