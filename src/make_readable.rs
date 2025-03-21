pub fn make_readable(seconds: u32) -> String {
    let hours = seconds / 60 / 60;
    let minutes = seconds / 60 % 60;
    let seconds = seconds % 60;
    
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}
