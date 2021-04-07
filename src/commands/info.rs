use clap::ArgMatches;
use crate::error::GitCFError;
use crate::settings::Settings;

pub fn execute_info(_: &ArgMatches) -> Result<(), GitCFError> {
    let settings = Settings::load_file()?;

    settings.print_config()?;

    Ok(())
}
