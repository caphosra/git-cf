use clap::ArgMatches;
use crate::settings::Settings;

pub fn execute_config(_: &ArgMatches) -> Result<(), String> {
    let settings = Settings::load_file()?;

    settings.print_config()?;

    Ok(())
}
