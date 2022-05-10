use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const SPECIALCHARS: &str = "!§$%&/()=?{[]}+*~#-_,;.:<>|";

pub fn generate_random_password(options: &PasswordOptions) -> String {
    let mut result: Vec<char> = Vec::new();
    if options.is_empty() {
        return result.iter().collect();
    }

    let mut chars = String::new();

    let mut char: char;
    let mut index;

    if options.upper_case {
        index = rand::thread_rng().gen_range(0..(LETTERS.len() - 1));
        char = LETTERS.to_uppercase().chars().nth(index).unwrap();
        chars.push_str(&LETTERS.to_uppercase());
        result.push(char);
    }

    if options.lower_case {
        index = rand::thread_rng().gen_range(0..(LETTERS.len() - 1));
        char = LETTERS.chars().nth(index).unwrap();
        chars.push_str(LETTERS);
        result.push(char);
    }

    if options.numeric {
        index = rand::thread_rng().gen_range(0..(NUMBERS.len() - 1));
        char = NUMBERS.chars().nth(index).unwrap();
        chars.push_str(NUMBERS);
        result.push(char);
    }

    if options.special {
        index = rand::thread_rng().gen_range(0..(SPECIALCHARS.len() - 1));
        char = SPECIALCHARS.chars().nth(index).unwrap();
        chars.push_str(SPECIALCHARS);
        result.push(char);
    }

    loop {
        if result.len() >= options.lenght {
            break;
        }

        index = rand::thread_rng().gen_range(0..(chars.len() - 1));
        char = chars.chars().nth(index).unwrap();
        result.push(char);
    }
    result.shuffle(&mut thread_rng());
    result.iter().collect()
}
pub struct PasswordOptions {
    upper_case: bool,
    lower_case: bool,
    numeric: bool,
    special: bool,
    lenght: usize,
}

impl PasswordOptions {
    pub fn new(options: u8, mut lenght: usize) -> Self {
        let options = u8_to_array(options);
        if lenght < 4 {
            lenght = 4;
        }
        Self {
            lenght,
            upper_case: options[0],
            lower_case: options[1],
            numeric: options[2],
            special: options[3],
        }
    }

    fn is_empty(&self) -> bool {
        !self.upper_case && !self.lower_case && !self.numeric && !self.special
    }
}

fn u8_to_array(mut value: u8) -> [bool; 4] {
    let mut result: [bool; 4] = [false; 4];
    let mut index = 0;
    if value > 15 {
        value = 15;
    }
    loop {
        if value == 0 {
            break;
        }
        if (value % 2) == 1 {
            result[index] = true;
        }
        index += 1;
        value /= 2;
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! try_parse_u8 {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let arr = u8_to_array(input);
                assert_eq!(arr, expected);
            }
        )*
        }
    }

    try_parse_u8! {
        parse_0_to_array: (0, [false, false, false, false]),
        parse_6_to_array: (6, [false, true, true, false]),
        parse_9_to_array: (9, [true, false, false, true]),
        parse_11_to_array: (11, [true, false, true, true]),
        parse_15_to_array: (15, [true, true, true, true]),
        parse_20_to_array: (20, [true, true, true, true]),
    }
}