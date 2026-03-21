use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use shellexpand;

pub fn save_token_to_file(json: &String, file_path: &str) -> std::io::Result<()> {
    let expanded_path = shellexpand::tilde(file_path).into_owned();
    let path = PathBuf::from(&expanded_path);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // let json = serde_json::to_string(token_info)?;
    let mut file = File::create(&expanded_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_token_from_file(file_path: &str) -> std::io::Result<Option<String>> {
    let expanded_path = shellexpand::tilde(file_path);
    let mut file = match File::open(expanded_path.as_ref()) {
        Ok(file) => file,
        Err(_) => return Ok(None),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(Some(contents))
    // let token_info: TokenInfo = serde_json::from_str(&contents)?;
    // Ok(Some(token_info))
}