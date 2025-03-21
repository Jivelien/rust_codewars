pub fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    
    for val in lst.to_vec() {
        if count_value_in_vec(val, result.clone()) < n {
            result.push(val);
        }
    }
    result
}

pub fn count_value_in_vec(value: u8, vector: Vec<u8>) -> usize {
    vector.iter().filter(|&&x| x == value).count()
}