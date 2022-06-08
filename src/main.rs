use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use std::process::Command;

use clap::Error;
use clap::Parser;
use dialoguer::Input;
use dialoguer::MultiSelect;

use chrono::prelude::*;
use chrono::Duration;
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

fn get_yesterday(mut data_dir: PathBuf) -> Option<PathBuf> {
    // 1. look for yesterday
    let yesterday_local: Date<Local> = Local::today() - Duration::days(1);
    let yesterday_local_str = yesterday_local.format("%0e%b%Y.txt").to_string();
    data_dir.push(yesterday_local_str);
    if data_dir.as_path().exists() {
        return Some(data_dir);
    }

    // 2. if not found search the whole dir

    None
}

fn main() {
    setup().unwrap();

    let cfg: NdayConfig = confy::load("nday").unwrap();

    let local: Date<Local> = Local::today();
    let mut file_path_buf = cfg.dir.clone();
    file_path_buf.push(local.format("%0e%b%Y.txt").to_string());
    let file_path = file_path_buf.as_path();
    let file_path_str = file_path_buf.display();

    match File::open(&file_path) {
        Err(_) => match File::create(&file_path) {
            Err(why) => {
                panic!("couldn't write to {}: {}", file_path_str, why)
            }

            Ok(mut new_file) => {
                let data_path_buf = cfg.dir.clone();
                let kicked_items_to_add = match get_yesterday(data_path_buf) {
                    Some(yesterday_file_path) => {
                        println!("pulling from yesterday: {}", yesterday_file_path.display());
                        let yesterday_file = File::open(&yesterday_file_path).unwrap();
                        let kicked_items = parse_kicked(yesterday_file);

                        let mut kicked_items_to_copy: Vec<String> = Vec::new();
                        if kicked_items.len() > 0 {
                            let chosen: Vec<usize> =
                                MultiSelect::new().items(&kicked_items).interact().unwrap();

                            for s in chosen {
                                kicked_items_to_copy.push(kicked_items[s].clone());
                            }
                        }
                        kicked_items_to_copy.push("- ".to_string());
                        kicked_items_to_copy
                    }
                    _ => {
                        let mut kicked_items_to_copy: Vec<String> = Vec::new();
                        kicked_items_to_copy.push("- ".to_string());
                        kicked_items_to_copy
                    }
                };

                let todays_text = local.format("%-e %B, %Y").to_string();
                let todo_text = format!("todo:\n{}", kicked_items_to_add.join("\n"));
                let done_text = "done:\n- ";
                let kicked_text = "kicked:\n- ";
                let new_file_text = format!(
                    "{}\n\n{}\n\n{}\n\n{}\n",
                    todays_text, todo_text, done_text, kicked_text
                );
                match new_file.write_all(new_file_text.as_bytes()) {
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

    Command::new(cfg.tool)
        .arg(file_path_str.to_string())
        .status()
        .unwrap();
}
