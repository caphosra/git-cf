use std::{fs::File, io::Read};
use serde_json::to_string;
use sha2::{ Digest, Sha256 };
use crate::settings::Settings;

fn digest(buffer: &[u8]) -> Result<String, String> {
    let result = Sha256::digest(buffer);
    let result = format!("{:X}", result);

    Ok(result)
}

pub fn digest_file(path: &String) -> Result<String, String> {
    let mut file = File::open(&path)
        .map_err(|err| {
            err.to_string()
        })?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .map_err(|err| {
            err.to_string()
        })?;

    digest(buffer.as_bytes())
}

pub fn digest_settings(settings: &Settings) -> Result<String, String> {
    let settings_buffer = to_string(settings)
        .map_err(|err| {
            err.to_string()
        })?;

    digest(settings_buffer.as_bytes())
}