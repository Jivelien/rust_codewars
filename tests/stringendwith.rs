use rust_codewars::stringendwith::*;

#[test]
fn test_reverse_abc_become_cba() {
    assert_eq!("cba", reverse("abc".to_string()))
}
#[test]
fn test_reverse_feg_become_gef() {
    assert_eq!("feg", reverse("gef".to_string()))
}

#[test]
fn test_get_1_char_at_end_of_abc() {
    assert_eq!("c", get_n_char_at_end("abc", 1))
}

#[test]
fn test_get_1_char_at_end_of_def() {
    assert_eq!("f", get_n_char_at_end("def", 1))
}

#[test]
fn returns_expected() {
    assert_eq!(true, solution("abc", "c"));
}

#[test]
fn returns_expected2() {
    assert_eq!(false, solution("strawberry", "banana"));
}
