use clap::ArgMatches;
use crate::compressor::compress;
use crate::settings::Settings;

pub fn execute_pack(_: &ArgMatches) -> Result<(), String> {
    let settings = Settings::load_file()?;
    match settings.validate()? {
        Some(_) => {
            error!("The information is not up to date. Please run `git cf update` first.");
        },
        None => {
            info!("The information is up to date. Start to compress them.");
            compress(&settings)?;
        }
    }

    Ok(())
}
