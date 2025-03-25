pub fn rot13(message: &str) -> String {
    message.chars().map(|c| rotation_13(c)).collect()
}

pub fn rotation_13(a_char: char) -> char {
    match a_char as u8{
        b'A'..=b'Z' => (((a_char as u8 + 13 - 65) % 26) + 65)  as char,
        b'a'..=b'z' =>(((a_char as u8 + 13 - 97) % 26) + 97)  as char,
        _ => a_char
    }
}