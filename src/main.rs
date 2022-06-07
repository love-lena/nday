use std::path::PathBuf;

use console::Term;
use clap::Parser;

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
        println!("Running setup...");
        let mut homepath = home::home_dir().unwrap();
        homepath.push("nday");
        cfg.dir = homepath;
        cfg.setup = true;
        confy::store("nday", cfg).unwrap();
    }

    let term = Term::stdout();
    term.write_line("Hello World!")?;

    let newcfg: NdayConfig = confy::load("nday").unwrap();
    println!("{}", newcfg.dir.display());

    term.clear_line()?;

    return Ok(());
}
