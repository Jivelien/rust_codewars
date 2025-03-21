pub fn tower_builder(n_floors: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    
    let nb_char: usize = n_floors* 2 - 1;
    
    for floor in 1..=n_floors {
        let nb_star = floor*2-1;
        result.push(format!("{: ^nb_char$}", "*".repeat(nb_star)));
    }
    result
}
