use rust_codewars::tower_builder::*;


#[test]
fn fixed_tests() {
    assert_eq!(tower_builder(1), vec!["*"]);
}

#[test]
fn fixed_tests2() {
    assert_eq!(tower_builder(2), vec![" * ", "***"]);
}

#[test]
fn fixed_tests3() {
    assert_eq!(tower_builder(3), vec!["  *  ", " *** ", "*****"]);
}
