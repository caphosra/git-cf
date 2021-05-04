use clap::ArgMatches;
use glob::glob;
use crate::error::GitCFError;
use crate::settings::Settings;

pub fn execute_add(matches: &ArgMatches) -> Result<(), GitCFError> {
    let mut settings = Settings::load_file()?;

    assert_ne!(matches.value_of("FILE"), None);

    let file_glob = matches.value_of("FILE").unwrap().to_string();
    for file in glob(&file_glob)? {
        let file = format!("{}", file?.display());
        settings.files.push(file);
    }

    match settings.validate()? {
        Some(settings) => {
            info!("Hashes are no longer matched. Try to update them...");
            settings.save_file()?;
        },
        None => {
            info!("No need to update them.");
        }
    }

    Ok(())
}
