use clap::ArgMatches;
use crate::error::GitCFError;
use crate::settings::Settings;

pub fn execute_init(_: &ArgMatches) -> Result<(), GitCFError> {
    let settings = Settings::new(vec![], vec![]);

    settings.save_file()?;

    Ok(())
}
