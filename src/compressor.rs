use std::fs::File;
use std::io::{ Read, Write };
use zip::*;
use zip::write::*;
use crate::hash::digest_settings;
use crate::settings::Settings;

pub fn compress(settings: &Settings) -> Result<(), String> {
    let zip_file_path = format!(
        "./patch-{}.zip",
        digest_settings(settings)?
    );

    let mut zip_file = File::create(zip_file_path)
        .map_err(|err| {
            err.to_string()
        })?;

    let mut zip = ZipWriter::new(&mut zip_file);
    let options = FileOptions::default().compression_method(CompressionMethod::Stored);

    for file in &settings.files {
        zip.start_file(file, options)
            .map_err(|err| {
                err.to_string()
            })?;

        let mut file = File::open(file)
            .map_err(|err| {
                err.to_string()
            })?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|err| {
                err.to_string()
            })?;

        zip.write(&buffer)
            .map_err(|err| {
                err.to_string()
            })?;
    }

    zip.finish()
        .map_err(|err| {
            err.to_string()
        })?;

    Ok(())
}
