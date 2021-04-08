use std::io::{ stdin, Write };
use clap::ArgMatches;
use termcolor::*;
use crate::compressor::extract;
use crate::error::GitCFError;
use crate::settings::Settings;

pub fn execute_patch(_: &ArgMatches) -> Result<(), GitCFError> {
    let settings = Settings::load_file()?;

    loop {
        println!("Are you sure? Please answer yes or no > ");

        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;

        buffer = buffer.trim_end().to_string();

        if buffer == "yes" {
            break;
        }
        else if buffer == "no" {
            return Ok(())
        }
        else {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);

            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
            writeln!(&mut stdout, "Don't joke (>_<)")?;
            stdout.reset()?;
        }
    }

    extract(&settings)?;

    Ok(())
}
