use std::fs;
use regex::Regex;
use log::debug;

pub fn remove_integrity_attribute() -> Result<(), Box<dyn std::error::Error>> {
    let html_files = fs::read_dir("dist")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "html"));

    for file in html_files {
        let file_path_buf = file.path();
        let file_path = file_path_buf.to_str().ok_or("Could not convert file path to string")?;

        let content = fs::read_to_string(file_path)?;
        let re = Regex::new(r#" integrity="[^"]*""#)?;
        let new_content = re.replace_all(&content, "");
        fs::write(&file_path_buf, new_content.as_bytes())?;
    }
    debug!("Integrity attribute removed successfully.");
    Ok(())
}
