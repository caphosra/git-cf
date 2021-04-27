use std::{fs::File, io::Read};
use serde_json::to_string;
use sha2::{ Digest, Sha256 };
use crate::error::GitCFError;
use crate::settings::Settings;

fn digest(buffer: &[u8]) -> Result<String, GitCFError> {
    let result = Sha256::digest(buffer);
    let result = format!("{:X}", result);

    Ok(result)
}

pub fn digest_file(path: &String) -> Result<String, GitCFError> {
    let mut file = File::open(&path)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    digest(&buffer)
}

pub fn digest_settings(settings: &Settings) -> Result<String, GitCFError> {
    let settings_buffer = to_string(settings)?;

    digest(settings_buffer.as_bytes())
}

pub fn get_patch_name(settings: &Settings) -> Result<String, GitCFError> {
    let digest = digest_settings(settings)?;

    Ok(format!("./patch-{}.zip", &digest[..10]))
}
