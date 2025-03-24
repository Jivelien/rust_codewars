use std::collections::HashMap;

pub fn cakes(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>) -> u32 {
    let mut potential_cake: HashMap<&str, u32> = HashMap::new();
    
    for ingredients in recipe.keys() {
        match available.contains_key(ingredients) {
            true => { potential_cake.insert(ingredients, available[ingredients] / recipe[ingredients]) ; },
            false => return 0,
        };
    }

    let mut result: u32 = 99;
    for ingredient in potential_cake.keys() {
        if potential_cake[ingredient] < result {
            result = potential_cake[ingredient];
        }
    }
    result
}
