use clap::ArgMatches;
use crate::settings::Settings;

pub fn execute_update(_: &ArgMatches) -> Result<(), String> {
    let settings = Settings::load_file()?;
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
