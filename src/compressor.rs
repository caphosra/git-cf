use std::fs::{ OpenOptions, File };
use std::io::{ copy, Read, Write };
use std::path::Path;
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

pub fn extract(settings: &Settings) -> Result<(), GitCFError> {
    let zip_path = get_patch_name(settings)?;
    let zip_path = Path::new(&zip_path);

    if zip_path.exists() {
        let mut zip_file = File::open(zip_path)?;
        let mut zip = ZipArchive::new(&mut zip_file)?;

        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            info!("Extracting {}...", file.name());

            let mut bytes: Vec<u8> = Vec::new();
            copy(&mut file, &mut bytes)?;

            let mut out_file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(format!("./{}", file.name()))?;
            out_file.write_all(&bytes)?;
        }

        Ok(())
    }
    else {
        Err(GitCFError::StringError(
            format!("Not found \"{}\"", &get_patch_name(settings)?)
        ))
    }
}
