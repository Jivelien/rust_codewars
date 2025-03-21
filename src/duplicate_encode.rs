pub fn duplicate_encode(word: &str) -> String {
    let lower_case_word = &*word.to_ascii_lowercase();
    let mut result = String::new();
    
    for letter in lower_case_word.chars() {
        match count_letter(lower_case_word, letter) {
            1 => result.push_str("("),
            _ => result.push_str(")")
        }
    }
    result
}

pub fn count_letter(word: &str, letter: char) -> i8 {
    word.chars().filter(|c| *c == letter).count() as i8
}
