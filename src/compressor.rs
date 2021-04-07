use std::fs::File;
use std::io::{ Read, Write };
use zip::*;
use zip::write::*;
use crate::error::GitCFError;
use crate::hash::get_patch_name;
use crate::settings::Settings;

pub fn compress(settings: &Settings) -> Result<(), GitCFError> {
    let mut zip_file = File::create(get_patch_name(settings)?)?;

    let mut zip = ZipWriter::new(&mut zip_file);
    let options = FileOptions::default().compression_method(CompressionMethod::Stored);

    for file in &settings.files {
        zip.start_file(file, options)?;

        let mut file = File::open(file)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        zip.write(&buffer)?;
    }

    zip.finish()?;

    Ok(())
}
