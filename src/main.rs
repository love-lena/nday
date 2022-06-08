use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use std::process::Command;

use clap::Parser;
use dialoguer::Input;
use dialoguer::MultiSelect;

use chrono::prelude::*;
use chrono::Local;

use serde::{Deserialize, Serialize};

// Default location is /Users/lena/Library/Preferences/rs.nday/
#[derive(Serialize, Deserialize)]
struct NdayConfig {
    #[serde(default)]
    dir: PathBuf,
    tool: String,
    setup: bool,
}

/// `NdayConfig` implements `Default`
impl ::std::default::Default for NdayConfig {
    fn default() -> Self {
        let mut homepath = home::home_dir().unwrap();
        homepath.push("nday");
        return Self {
            dir: homepath,
            tool: String::from("vim"),
            setup: false,
        };
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Run first time setup
    #[clap(short, long)]
    setup: bool,
}

static TEMPLATE_TEXT: &str = "\n\nTo-do today:\n- \n\nDone today:\n- \n\nKicked to tomorrow:\n- \n";

fn setup() -> Result<(), ::std::io::Error> {
    let args = Args::parse();

    let mut cfg: NdayConfig = confy::load("nday").unwrap();

    if !cfg.setup || args.setup {
        let dir_str = cfg.dir.into_os_string().into_string().unwrap();
        let tool_str = cfg.tool;

        let dir_input: String = Input::new()
            .with_prompt("Enter notes path")
            .default(dir_str)
            .interact_text()?;

        let tool_input: String = Input::new()
            .with_prompt("Enter tool to edit notes")
            .default(tool_str)
            .interact_text()?;

        let mut datapath = PathBuf::new();
        datapath.push(dir_input);
        let datapath_path = datapath.as_path();

        fs::create_dir_all(datapath_path)?;

        cfg.dir = datapath;
        cfg.tool = tool_input;
        cfg.setup = true;
        confy::store("nday", cfg).unwrap();
    }

    return Ok(());
}

// TODO: return result
fn parse_kicked(mut file: File) -> Vec<String> {
    let mut kicked_items: Vec<String> = Vec::new();

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let split = s.split("\n");
    let mut start_collecting = false;
    for s in split {
        if start_collecting && !s.is_empty() {
            kicked_items.push(s.to_string());
        }
        if s.eq("Kicked to tomorrow:") {
            start_collecting = true;
        }
    }

    return kicked_items;
}

fn main() {
    setup().unwrap();

    let mut cfg: NdayConfig = confy::load("nday").unwrap();

    let local: Date<Local> = Local::today();
    cfg.dir.push(local.format("%0e%b%Y.txt").to_string());
    let file_path = cfg.dir.as_path();
    let file_path_str = cfg.dir.display();

    let today_file = match File::open(&file_path) {
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
            println!("opening today's notes in {} ({})", cfg.tool, file_path_str);
            file
        }
    };

    let kicked_items = parse_kicked(today_file);

    if kicked_items.len() > 0 {
        let chosen: Vec<usize> = MultiSelect::new().items(&kicked_items).interact().unwrap();

        for s in chosen {
            println!("{}", kicked_items[s]);
        }
    }

    Command::new(cfg.tool)
        .arg(file_path_str.to_string())
        .status()
        .unwrap();
}
