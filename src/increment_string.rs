pub fn increment_string(s: &str) -> String {
    let string_part = remove_number(s);
    let number_part = extract_number(s);

    let number_plus_one = add_one(&number_part);

    format!("{}{}", string_part, number_plus_one).to_string()
}

pub fn extract_number(s: &str) -> String {
    let chars_in_str = s.chars().rev();
    let mut result = Vec::new();

    for c in chars_in_str {
        if c.is_digit(10) {
            result.push(c);
        } else {
            break;
        }
    }
    if result.is_empty() {
        return String::from("0");
    }
    result.reverse();
    result.iter().collect()
}

pub fn add_one2(s: &str) -> String {
    let parsed = s.parse::<u128>();
    let num = match parsed {
        Ok(v) => v + 1,
        Err(_) => 0,
    };

    let nb_char = s.len();
    format!("{:0nb_char$}", num).to_string()
}

pub fn add_one(s: &str) -> String {
    let num_chars = s.chars().rev();

    let mut retenu = true;
    let mut result = Vec::new();
    for c in num_chars {
        if retenu {
            let new_c = c.to_digit(10).unwrap() + 1;
            if new_c == 10 {
                result.push('0');
            } else {
                result.push(format!("{new_c}").chars().next().unwrap());
                retenu = false;
            }
        } else {
            result.push(c);
        }
    }
    if retenu {
        result.push('1');
    }

    result.reverse();
    result.iter().collect()
}

pub fn remove_number(s: &str) -> String {
    let chars_in_str: Vec<char> = s.chars().rev().collect();
    let mut result: Vec<char> = Vec::new();

    let mut state = true;

    for char in chars_in_str {
        if char.is_digit(10) & state == true {
            continue;
        }
        state = false;
        result.push(char);
    }
    result.reverse();
    result.iter().collect()
}
