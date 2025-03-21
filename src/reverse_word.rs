pub fn reverse_words(str: &str) -> String {
    separate_word(str)
        .iter()
        .map(|x| format!("{} ", reverse_one_word(x)))
        .collect::<String>()
        .trim()
        .to_string()
}

pub fn separate_word(sentence: &str) -> Vec<&str> {
    let words = sentence.split(" ");
    Vec::from_iter(words)
}
pub fn reverse_one_word(word: &str) -> String {
    word.chars().rev().collect::<String>()
}
