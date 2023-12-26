const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    let mut initial_string = String::from("hello, world");
    match pig_latin(&initial_string) {
        None => println!("empty string"),
        Some(pig_latin) => println!("pig latin of {} is {}", initial_string, pig_latin),
    }

    initial_string = String::from("Здравствуйте");
    match pig_latin(&initial_string) {
        None => println!("empty string"),
        Some(pig_latin) => println!("pig latin of {} is {}", initial_string, pig_latin),
    }
}

fn pig_latin(s: &String) -> Option<String> {
    if s.len() == 0 {
        return None;
    }

    if is_vowel(s.chars().nth(0).unwrap()) {
        return format!("{}-{}ay", s, 'h').into();
    }

    let mut chars = s.chars();
    let first_char = chars.nth(0).unwrap();
    chars.next();
    return format!("{}-{}ay", chars.as_str(), first_char).into();
}

fn is_vowel(c: char) -> bool {
    for v in VOWELS {
        if c != v {
            return false;
        }
    }

    true
}
