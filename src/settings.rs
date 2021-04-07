use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{ Deserialize, Serialize };
use serde_json::{ from_str, to_string };
use termcolor::*;
use crate::error::GitCFError;
use crate::hash::{ digest_file, get_patch_name };

const SETTINGS_FILE_NAME: &str = "./git-cf.json";

#[derive(Deserialize, Debug, Serialize)]
pub struct Settings {
    pub files: Vec<String>,
    pub hashes: Vec<String>
}

impl Settings {
    pub fn new(files: Vec<String>, hashes: Vec<String>) -> Self {
        Settings {
            files, hashes
        }
    }

    pub fn load_file() -> Result<Settings, GitCFError> {
        let path = Path::new(SETTINGS_FILE_NAME);
        if !path.exists() {
            Err(GitCFError::StringError(
                format!("Not found \"{}\" here.", SETTINGS_FILE_NAME)
            ))
        }
        else {
            let mut settings_file = File::open(path)?;
            let mut settings_buffer = String::new();
            settings_file.read_to_string(&mut settings_buffer)?;

            debug!("Read \"{}\" successfully.", SETTINGS_FILE_NAME);

            let result: Settings = from_str(settings_buffer.as_str())?;

            debug!("Parsed \"{}\" successfully.", SETTINGS_FILE_NAME);

            Ok(result)
        }
    }

    pub fn save_file(&self) -> Result<(), GitCFError> {
        let path = Path::new(SETTINGS_FILE_NAME);
        let mut settings_file = File::create(path)?;
        let settings_buffer = to_string(self)?;

        debug!("Parsed \"{}\" successfully.", SETTINGS_FILE_NAME);

        settings_file.write_all(settings_buffer.as_bytes())?;

        debug!("Wrote \"{}\" successfully.", SETTINGS_FILE_NAME);

        Ok(())
    }

    pub fn validate(&self) -> Result<Option<Settings>, GitCFError> {
        if self.files.len() != self.hashes.len() {
            let mut hashes: Vec<String> = Vec::new();
            for file in self.files.iter() {
                let digest = digest_file(&file)?;
                hashes.push(digest);
            }

            Ok(Some(Settings::new(self.files.clone(), hashes)))
        }
        else {
            let mut same = true;
            let mut hashes: Vec<String> = Vec::new();
            for (file, hash) in self.files.iter().zip(&self.hashes) {
                let digest = digest_file(&file)?;
                if hash.ne(&digest) {
                    same = false;
                }
                hashes.push(digest);
            }
            if same {
                Ok(None)
            }
            else {
                Ok(Some(Settings::new(self.files.clone(), hashes)))
            }
        }
    }

    pub fn print_config(&self) -> Result<(), GitCFError> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        write!(&mut stdout, "Patch name: ")?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        writeln!(&mut stdout, "{}", get_patch_name(self)?)?;
        stdout.reset()?;

        write!(&mut stdout, "Up to date: ")?;

        match self.validate()? {
            Some(_) => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                writeln!(stdout, "NO")?;
                stdout.reset()?;
            },
            None => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                writeln!(stdout, "YES")?;
                stdout.reset()?;
            }
        }

        writeln!(&mut stdout, "[Files]")?;

        for (file, hash) in self.files.iter().zip(&self.hashes) {
            let digest = digest_file(&file)?;

            write!(&mut stdout, "- ")?;

            let mut color = Color::Red;
            if digest.eq(hash) {
                color = Color::Green;
            }
            stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
            writeln!(stdout, "{}", file)?;
            stdout.reset()?;

            writeln!(&mut stdout, " ({})", digest)?;
        }

        Ok(())
    }
}
