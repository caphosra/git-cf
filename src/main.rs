extern crate clap;
extern crate glob;
#[macro_use]
extern crate log;
extern crate serde_json;
extern crate serde;
extern crate sha2;
extern crate simplelog;
extern crate termcolor;
extern crate zip;

use clap::*;
use simplelog::*;
use crate::commands::add::execute_add;
use crate::commands::info::execute_info;
use crate::commands::init::execute_init;
use crate::commands::pack::execute_pack;
use crate::commands::patch::execute_patch;
use crate::commands::remove::execute_remove;
use crate::commands::update::execute_update;

const APP_NAME: &str = "git-cf";
const AUTHOR_NAME: &str = "Akihisa Yagi <capra314cabra@gmail.com>";

fn main() {
    let matches = App::new(APP_NAME)
        .version(crate_version!())
        .author(AUTHOR_NAME)
        .about("This is a git extension which compress binaries, which is not familiar with git into one file.")
        .subcommand(
            App::new("add")
                .about("Adds file to \"git-cf.json\"")
                .version(crate_version!())
                .author(AUTHOR_NAME)
                .arg(
                    Arg::with_name("FILE")
                        .required(true)
                        .help("A file which is going to be added")
                )
        )
        .subcommand(
            App::new("info")
                .about("Prints the information")
                .version(crate_version!())
                .author(AUTHOR_NAME)
        )
        .subcommand(
            App::new("init")
                .about("Initializes this project")
                .version(crate_version!())
                .author(AUTHOR_NAME)
        )
        .subcommand(
            App::new("pack")
                .about("Compresses binaries into one file")
                .version(crate_version!())
                .author(AUTHOR_NAME)
        )
        .subcommand(
            App::new("patch")
                .about("Extracts binaries")
                .version(crate_version!())
                .author(AUTHOR_NAME)
        )
        .subcommand(
            App::new("remove")
                .about("Removes file to \"git-cf.json\"")
                .version(crate_version!())
                .author(AUTHOR_NAME)
                .arg(
                    Arg::with_name("FILE")
                        .required(true)
                        .help("A file which is going to be removed")
                )
        )
        .subcommand(
            App::new("update")
                .about("Updates \"git-cf.json\" with infos")
                .version(crate_version!())
                .author(AUTHOR_NAME)
        )
        .get_matches();

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
        ]
    ).unwrap();

    if let Some(matches) = matches.subcommand_matches("add") {
        match execute_add(matches) {
            Ok(_) => { },
            Err(msg) => {
                error!("{}", msg);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("info") {
        match execute_info(matches) {
            Ok(_) => { },
            Err(msg) => {
                error!("{}", msg);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("init") {
        match execute_init(matches) {
            Ok(_) => { },
            Err(msg) => {
                error!("{}", msg);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("pack") {
        match execute_pack(matches) {
            Ok(_) => { },
            Err(msg) => {
                error!("{}", msg);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("patch") {
        match execute_patch(matches) {
            Ok(_) => { },
            Err(msg) => {
                error!("{}", msg);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        match execute_remove(matches) {
            Ok(_) => { },
            Err(msg) => {
                error!("{}", msg);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("update") {
        match execute_update(matches) {
            Ok(_) => { },
            Err(msg) => {
                error!("{}", msg);
            }
        }
    }
}

pub mod commands;
pub mod compressor;
pub mod error;
pub mod hash;
pub mod settings;
