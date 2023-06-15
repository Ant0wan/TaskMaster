use std::fs;

fn recognize_format(file_path: &str) -> String {
    let contents: String = fs::read_to_string(file_path).expect("Unable to read file");

    // Check if file is in INI format
    if contents.starts_with('[') {
        return String::from("INI format");
    }

    // Check if file is in YAML format
    match serde_yaml::from_str::<serde_yaml::Value>(&contents) {
        Ok(_) => String::from("YAML format"),
        Err(_) => String::from("Unknown format"),
    }
}
