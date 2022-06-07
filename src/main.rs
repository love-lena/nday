use std::path::PathBuf;
use std::fs;

use clap::Parser;
use dialoguer::Input;

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
    /// Name of the person to greet
    #[clap(short, long)]
    setup: bool,
}

fn main() -> Result<(), ::std::io::Error> {

    let args = Args::parse();

    // Default location is /Users/lena/Library/Preferences/rs.nday/
    let mut cfg: NdayConfig = confy::load("nday").unwrap();

    if !cfg.setup || args.setup {
        let mut homepath = home::home_dir().unwrap();
        homepath.push("nday");
        let homepath_str = homepath.into_os_string().into_string().unwrap();
        
        let input : String = Input::new()
            .with_prompt("Enter default path")
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

    // let newcfg: NdayConfig = confy::load("nday").unwrap();

    return Ok(());
}
