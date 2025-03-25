use rust_codewars::rot13::*;

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
// ROT13 is a simple letter substitution cipher that replaces a letter with the letter 13 letters after it in the alphabet. ROT13 is an example of the Caesar cipher.
// 
// Create a function that takes a string and returns the string ciphered with Rot13. 
// If there are numbers or special characters included in the string, 
// they should be returned as they are. Only letters from the latin/english 
// alphabet should be shifted, like in the original Rot13 "implementation".
const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

fn dotest(s: &str, expected: &str) {
    assert_eq!(rot13(s), expected, "{ERR_MSG} with message = \"{s}\"")
}

#[test]
fn sample_tests() {
    dotest("test", "grfg");
    dotest("Test", "Grfg");
}

#[test]
fn test_char_to_integer() {
    assert_eq!('a' as u8,97);
    assert_eq!('z' as u8,122);
    assert_eq!('A' as u8,65);
    assert_eq!('Z' as u8,90);
}

#[test]
fn test_rotation_13() {
    assert_eq!(rotation_13('a'),'n');
    assert_eq!(rotation_13('z'),'m');
    assert_eq!(rotation_13('A'),'N');
    assert_eq!(rotation_13('Z'),'M');
}