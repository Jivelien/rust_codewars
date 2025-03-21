use rust_codewars::make_readable::*;

const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

fn dotest(s: u32, expected: &str) {
    assert_eq!(make_readable(s), expected, "{ERR_MSG} with seconds = {s}")
}

#[test]
fn fixed_tests() {
    dotest(0, "00:00:00");
}
#[test]
fn fixed_tests2() {
    dotest(59, "00:00:59");
}

#[test]
fn fixed_tests_custom() {
    dotest(61, "00:01:01");
}
#[test]
fn fixed_tests3() {
    dotest(60, "00:01:00");
}
#[test]
fn fixed_tests4() {
    dotest(3599, "00:59:59");
}
#[test]
fn fixed_tests5() {
    dotest(3600, "01:00:00");
}
#[test]
fn fixed_tests6() {
    dotest(86399, "23:59:59");
}
#[test]
fn fixed_tests7() {
    dotest(86400, "24:00:00");
}
#[test]
fn fixed_tests8() {
    dotest(359999, "99:59:59");
}
