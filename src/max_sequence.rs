pub fn max_sequence(seq: &[i32]) -> i32 {
    let mut all_sum = Vec::from([0]);
    for (start_index, _) in seq.iter().enumerate() {
        for (end_index, _) in seq[start_index..].iter().enumerate() {
            let a_sum: i32 = seq[start_index..=start_index + end_index].iter().sum();
            all_sum.push(a_sum);
        }
    }
    *all_sum.iter().max().unwrap()
}
