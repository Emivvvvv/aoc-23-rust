use std::fs;

pub fn input(day: &str) -> String {
    let filepath = "inputs/input".to_string() + day +".txt";
    return read_file_string(filepath.as_str()).unwrap();
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}