use rand::Rng;
use rand::seq::SliceRandom;

use crate::data::string;
use crate::misc;

// letter will generate a single random lower case ASCII letter
pub fn letter() -> String {
    rand_letter()
}

fn rand_letter() -> String {
    String::from(misc::random_data(string::LOWER_CASE))
}

// letter_n will generate a random ASCII string with length N. Note that this function returns a string with a length of 1 when 0 is passed.
pub fn letter_n(n: i32) -> String {
    rand_letter_n(n)
}

fn rand_letter_n(n: i32) -> String {
    let mut values: String = String::from("");
    let counter: i32 = if n == 0 {1} else {n};

    for _ in 0..counter {
        values = values + &*rand_letter();
    }
    String::from(values)
}

// vowel will generate a single random lower case vowel
pub fn vowel() -> String {
    rand_vowel()
}

fn rand_vowel() -> String {
    String::from(misc::random_data(string::LOWER_VOWELS))
}

// Digit will generate a single ASCII digit
pub fn digit() -> String {
    rand_digit()
}

fn rand_digit() -> String {
    String::from(misc::random_data(string::DIGITS))
}

// digit_n will generate a random string of length N consists of ASCII digits. Note that the string generated can start with 0 and this function returns a string with a length of 1 when 0 is passed.
pub fn digit_n(n: i32) -> String {
    rand_digit_n(n)
}

fn rand_digit_n(n: i32) -> String {
    let mut values: String = String::from("");
    let counter: i32 = if n == 0 {1} else {n};

    for _ in 0..counter {
        values = values + &*rand_digit();
    }
    String::from(values)
}

// numerify will replace # with random numerical values
pub fn numerify(initial_string:String) -> String {
    misc::replace_with_numbers(initial_string)
}

// lexify will replace ? with random generated letters
pub fn lexify(initial_string:String) -> String {
    misc::replace_with_letter(initial_string)
}

// shuffle_strings will randomize a slice of strings
pub fn shuffle_strings(s: &mut [String]){
    let mut rng = rand::thread_rng();
    s.shuffle(&mut rng)
}

// random_string will take in a slice of string and return a randomly selected value
pub fn random_string(s: &mut [String]) -> String {
    return_random_string(s)
}

fn return_random_string(s: &mut [String]) -> String {
    let mut rng = rand::thread_rng();
    let rand_index = rng.gen_range(0, s.len()-1);
    s[rand_index].clone()
}

#[cfg(test)]
mod tests {
    use crate::string;
    use crate::testify::exec_mes;

    #[test]
    fn letter() {
        exec_mes("string::letter", || string::letter());
    }

    #[test]
    fn letter_n() {
        exec_mes("string::letter_n",|| string::letter_n(7));
        exec_mes("string::letter_n", || string::letter_n(3));
    }
    
    #[test]
    fn vowel() {
        exec_mes("string::vowel", || string::vowel());
    }
    
    #[test]
    fn digit() {
        exec_mes("string::digit", || string::digit());
    }

    #[test]
    fn digit_n() {
        exec_mes("string::digit_n", || string::digit_n(5));
        exec_mes("string::digit_n", || string::digit_n(11));
    }

    #[test]
    fn numerify() {
        exec_mes("string::numerify", || string::numerify("H#LL# W#RLD!!".to_owned()));
    }

    #[test]
    fn lexify() {
        exec_mes("string::lexify", || string::lexify("H?LL? W?RLD!!".to_owned()))
    }

    #[test]
    fn shuffle_strings() {
        let stable_strings: &mut [String] = &mut [
            "first".to_string(),
            "second".to_string(),
            "third".to_string(),
            "fourth".to_string(),
            "fifth".to_string()
        ];
        let test_strings: &mut [String] = &mut [
            "first".to_string(),
            "second".to_string(),
            "third".to_string(),
            "fourth".to_string(),
            "fifth".to_string()
        ];
        string::shuffle_strings(test_strings);
        println!("{:?}", test_strings);
        assert_ne!(stable_strings, test_strings)
    }

    #[test]
    fn random_string() {
        let stable_strings: &mut [String] = &mut [
            "first".to_string(),
            "second".to_string(),
            "third".to_string(),
            "fourth".to_string(),
            "fifth".to_string()
        ];
        let return_string: String = string::random_string(stable_strings);
        println!("return string: {:?}", return_string);
        assert!(stable_strings.contains(&return_string))
    }
}