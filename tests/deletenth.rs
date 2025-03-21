use rust_codewars::deletenth::*;

#[test]
fn test_basic() {
    assert_eq!(delete_nth(&[20, 37, 20, 21], 1), vec![20, 37, 21]);
}

#[test]
fn test_basic2 () {
    assert_eq!(delete_nth(&[1,1,3,3,7,2,2,2,2], 3), vec![1, 1, 3, 3, 7, 2, 2, 2]);
}

#[test]
fn test_count_value_in_vec() {
    let vec: Vec<u8> = vec![1,2,3];
    let sut = count_value_in_vec(1, vec);
    assert_eq!(1, sut);
}


#[test]
fn test_count_value_in_vec_2() {
    let vec: Vec<u8> = vec![1,1,3];
    let sut = count_value_in_vec(1, vec);
    assert_eq!(2, sut);
}