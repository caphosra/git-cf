use clap::ArgMatches;
use crate::error::GitCFError;
use crate::settings::Settings;

pub fn execute_add(matches: &ArgMatches) -> Result<(), GitCFError> {
    let mut settings = Settings::load_file()?;

    settings.files.push(matches.value_of("FILE").unwrap().to_string());

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
