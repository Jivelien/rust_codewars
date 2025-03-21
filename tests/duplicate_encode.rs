use rust_codewars::duplicate_encode::*;

#[test]
fn run_tests() {
    assert_eq!(duplicate_encode("din"),"(((");
    assert_eq!(duplicate_encode("recede"),"()()()");
    assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
    assert_eq!(duplicate_encode("(( @"),"))((");
}

#[test]
fn count_letter_return_the_number_of_times_a_letter_appear_in_a_string() {
    let sut = count_letter("abc", 'c');
    assert_eq!(1, sut);
}
#[test]
fn count_letter_return_the_number_of_times_a_letter_appear_in_a_string_2() {
    let sut = count_letter("efggc", 'g');
    assert_eq!(2, sut);
}