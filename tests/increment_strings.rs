use rust_codewars::increment_string::*;

fn dotest(s: &str, expected: &str) {
    let actual = increment_string(s);
    assert!(
        actual == expected,
        "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\""
    )
}

#[test]
fn sample_tests() {
    dotest("foo", "foo1");
    dotest("foobar001", "foobar002");
    dotest("foobar1", "foobar2");
    dotest("foobar00", "foobar01");
    dotest("foobar99", "foobar100");
    dotest("foobar099", "foobar100");
    dotest("", "1");
}

#[test]
fn extract_numbers_return_number_at_the_end_of_a_string() {
    let a_str = "abc1";
    let sut = extract_number(a_str);

    assert_eq!(sut, "1");
}

#[test]
fn extract_numbers_return_numbers_at_the_end_of_a_string() {
    let a_str = "cde123";
    let sut = extract_number(a_str);

    assert_eq!(sut, "123");
}

#[test]
fn extract_numbers_return_only_numbers_at_the_end_of_a_string() {
    let a_str = "c14de123";
    let sut = extract_number(a_str);

    assert_eq!(sut, "123");
}

#[test]
fn extract_numbers_return_0_if_no_number_in_string() {
    let a_str = "fyugbhjknjk";
    let sut = extract_number(a_str);

    assert_eq!(sut, "0");
}

#[test]
fn add_one_to_a_str() {
    let a_str = "1";
    let sut = add_one(a_str);

    assert_eq!(sut, "2");
}

#[test]
fn add_one_to_a_str2() {
    let a_str = "03";
    let sut = add_one(a_str);

    assert_eq!(sut, "04");
}

#[test]
fn remove_padding_number_in_string() {
    let a_str = "abc25";
    let sut = remove_number(a_str);
    assert_eq!(sut, "abc");
}

#[test]
fn remove_only_padding_number_in_string() {
    let a_str = "a1bc25";
    let sut = remove_number(a_str);
    assert_eq!(sut, "a1bc");
}

#[test]
fn abc() {
    let a_str = "";
    let sut = extract_number(a_str);
    assert_eq!(sut, "0");
}

#[test]
fn failling_test_when_sub() {
    let a_str = "HereComesATrickyTest99999999999999999999999999999999999999";
    let sut = increment_string(a_str);
    assert_eq!(sut, "HereComesATrickyTest100000000000000000000000000000000000000");
}
#[test]
fn failling_test_when_sub2() {
    let a_str = "Hj5p6g7wFCmeH64EWuq2xlYfaacDCpzRynEYsOZRm9vo9ZM9gJ9NKvUCzgxFTFxWk8uUxsGplXkPeYZd6YYk8SBasRvM8PRxJ2u606142999359397499759520969069589899798804791164786914879089734175629934990993516511945299";
    let sut = increment_string(a_str);
    assert_eq!(sut, "Hj5p6g7wFCmeH64EWuq2xlYfaacDCpzRynEYsOZRm9vo9ZM9gJ9NKvUCzgxFTFxWk8uUxsGplXkPeYZd6YYk8SBasRvM8PRxJ2u606142999359397499759520969069589899798804791164786914879089734175629934990993516511945300");
}