use rust_codewars::reverse_word::*;

// Rust tests
#[test]
fn sample_test() {
    assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god");
    assert_eq!(reverse_words("apple"), "elppa");
    assert_eq!(reverse_words("a b c d"),"a b c d");
    assert_eq!(reverse_words("double  spaced  words"), "elbuod  decaps  sdrow");
}

#[test]
fn test_separete_string_into_word() {
    let sentence = "a b c d";
    let sut = separate_word(sentence);
    assert_eq!(sut, vec!["a", "b", "c", "d"]);
}

#[test]
fn test_reverse_one_word() {
    let word = "hello";
    let sut = reverse_one_word(word);
    assert_eq!(sut, "olleh");
}
