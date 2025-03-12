pub fn solution(word: &str, ending: &str) -> bool {
    let end_of_word = get_n_char_at_end(word, ending.chars().count());
    end_of_word == ending
}

pub fn get_n_char_at_end(word: &str, n: usize) -> String {
    let reversed = reverse(word.to_string());
    let n_last: String = reversed.chars().take(n).collect();
    reverse(n_last)
}

pub fn reverse(word: String) -> String {
    word.chars().rev().collect::<String>()
}
