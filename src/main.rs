use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use std::process::Command;

use clap::Parser;
use dialoguer::Input;

use chrono::prelude::*;
use chrono::Local;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct NdayConfig {
    #[serde(default)]
    dir: PathBuf,
    setup: bool,
}

/// `NdayConfig` implements `Default`
impl ::std::default::Default for NdayConfig {
    fn default() -> Self {
        Self {
            dir: PathBuf::new(),
            setup: false,
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Should setup be performed?
    #[clap(short, long)]
    setup: bool,
}

static TEMPLATE_TEXT: &str = "\n\nTo-do today:\n- \n\nDone today:\n- \n\nKicked to tomorrow:\n- \n";

fn main() -> Result<(), ::std::io::Error> {
    let args = Args::parse();

    // Default location is /Users/lena/Library/Preferences/rs.nday/
    let mut cfg: NdayConfig = confy::load("nday").unwrap();

    if !cfg.setup || args.setup {
        let mut homepath = home::home_dir().unwrap();
        homepath.push("nday");
        let homepath_str = homepath.into_os_string().into_string().unwrap();

        let input: String = Input::new()
            .with_prompt("Enter notes path")
            .default(homepath_str)
            .interact_text()?;

        let mut datapath = PathBuf::new();
        datapath.push(input);
        let datapath_path = datapath.as_path();

        fs::create_dir_all(datapath_path)?;

        cfg.dir = datapath;
        cfg.setup = true;
        confy::store("nday", cfg).unwrap();
    }

    let mut updated_cfg: NdayConfig = confy::load("nday").unwrap();

    let local: Date<Local> = Local::today();
    updated_cfg
        .dir
        .push(local.format("%0e%b%Y.txt").to_string());
    let file_path = updated_cfg.dir.as_path();
    let file_path_str = updated_cfg.dir.display();

    let _file = match File::open(&file_path) {
        Err(_) => match File::create(&file_path) {
            Err(why) => {
                panic!("couldn't write to {}: {}", file_path_str, why)
            }
            Ok(mut new_file) => {
                let todays_text = local.format("%-e %B, %Y").to_string() + TEMPLATE_TEXT;
                match new_file.write_all(todays_text.as_bytes()) {
                    Err(why) => panic!("couldn't write to {}: {}", file_path_str, why),
                    Ok(_) => println!("created today's notes {}", file_path_str),
                };
                new_file
            }
        },

        Ok(file) => {
            println!("opening today's notes in vim ({})", file_path_str);
            file
        }
    };

    Command::new("vim")
        .arg(file_path_str.to_string())
        .status()
        .unwrap();

    return Ok(());
}
