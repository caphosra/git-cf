use clap::ArgMatches;
use crate::error::GitCFError;
use crate::settings::Settings;

pub fn execute_remove(matches: &ArgMatches) -> Result<(), GitCFError> {
    let mut settings = Settings::load_file()?;

    settings.files = settings.files
        .iter()
        .filter(|&file| {
            &matches.value_of("FILE").unwrap() != file
        })
        .cloned()
        .collect::<Vec<String>>();

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
