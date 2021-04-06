use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{ Deserialize, Serialize };
use serde_json::{ from_str, to_string };
use termcolor::*;
use crate::hash::digest_file;

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

    pub fn load_file() -> Result<Settings, String> {
        let path = Path::new(SETTINGS_FILE_NAME);
        if !path.exists() {
            Err(format!("Not found \"{}\" here.", SETTINGS_FILE_NAME))
        }
        else {
            let mut settings_file = File::open(path)
                .map_err(|err| {
                    err.to_string()
                })?;
            let mut settings_buffer = String::new();
            settings_file.read_to_string(&mut settings_buffer)
                .map_err(|err| {
                    err.to_string()
                })?;

            info!("Read \"{}\" successfully.", SETTINGS_FILE_NAME);

            let result: Settings = from_str(settings_buffer.as_str())
                .map_err(|err| {
                    err.to_string()
                })?;

            info!("Parsed \"{}\" successfully.", SETTINGS_FILE_NAME);

            Ok(result)
        }
    }

    pub fn save_file(&self) -> Result<(), String> {
        let path = Path::new(SETTINGS_FILE_NAME);
        let mut settings_file = File::create(path)
            .map_err(|err| {
                err.to_string()
            })?;
        let settings_buffer = to_string(self)
            .map_err(|err| {
                err.to_string()
            })?;

        info!("Parsed \"{}\" successfully.", SETTINGS_FILE_NAME);

        settings_file.write_all(settings_buffer.as_bytes())
            .map_err(|err| {
                err.to_string()
            })?;

        info!("Wrote \"{}\" successfully.", SETTINGS_FILE_NAME);

        Ok(())
    }

    pub fn validate(&self) -> Result<Option<Settings>, String> {
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

    pub fn print_config(&self) -> Result<(), String> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        writeln!(&mut stdout, "[Files]")
            .map_err(|err| {
                err.to_string()
            })?;

        for (file, hash) in self.files.iter().zip(&self.hashes) {
            let digest = digest_file(&file)?;

            write!(&mut stdout, "- ")
                .map_err(|err| {
                    err.to_string()
                })?;

            let mut color = Color::Red;

            if digest.eq(hash) {
                color = Color::Green;
            }

            stdout.set_color(
                    ColorSpec::new().set_fg(Some(color))
                )
                .map_err(|err| {
                    err.to_string()
                })?;

            write!(&mut stdout, "{}", file)
                .map_err(|err| {
                    err.to_string()
                })?;

            stdout.reset()
                .map_err(|err| {
                    err.to_string()
                })?;

            writeln!(&mut stdout, " ({})", digest)
                .map_err(|err| {
                    err.to_string()
                })?;
        }

        write!(&mut stdout, "Up to date : ")
            .map_err(|err| {
                err.to_string()
            })?;

        match self.validate()? {
            Some(_) => {
                stdout.set_color(
                        ColorSpec::new().set_fg(Some(Color::Red))
                    )
                    .map_err(|err| {
                        err.to_string()
                    })?;

                writeln!(&mut stdout, "NO")
                    .map_err(|err| {
                        err.to_string()
                    })?;

                stdout.reset()
                    .map_err(|err| {
                        err.to_string()
                    })?;
            },
            None => {
                stdout.set_color(
                        ColorSpec::new().set_fg(Some(Color::Green))
                    )
                    .map_err(|err| {
                        err.to_string()
                    })?;

                writeln!(&mut stdout, "YES")
                    .map_err(|err| {
                        err.to_string()
                    })?;

                stdout.reset()
                    .map_err(|err| {
                        err.to_string()
                    })?;
            }
        }

        Ok(())
    }
}
