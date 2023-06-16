use serde_ini;
use serde_yaml;
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq)]
pub enum FileFormat {
    Yaml,
    Ini,
}

pub fn recognize_file_format(filename: &str) -> Option<FileFormat> {
    if let Ok(mut file) = File::open(filename) {
        let mut contents = String::new();
        if let Ok(_) = file.read_to_string(&mut contents) {
            if is_yaml(&contents) {
                return Some(FileFormat::Yaml);
            } else if is_ini(&contents) {
                return Some(FileFormat::Ini);
            }
        }
    }
    None
}

fn is_yaml(contents: &str) -> bool {
    serde_yaml::from_str::<serde_yaml::Value>(contents).is_ok()
}

fn is_ini(contents: &str) -> bool {
    serde_ini::from_str::<serde_ini::Value>(&contents).is_ok()
}
