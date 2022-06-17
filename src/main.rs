use std::fs::read_dir;
use std::io::{Read, Write};
use std::vec;
use std::{fs::File, path::PathBuf};

use std::process::Command;

use chrono::{DateTime, Local, NaiveDate, ParseError};
use clap::Parser;
use console::style;
use dialoguer::Input;
use serde::{Deserialize, Serialize};

// Default location on mac is /Users/[user]/Library/Preferences/rs.nday/
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
        homepath.push("nday_data");
        Self {
            dir: homepath,
            tool: String::from("vim"),
            setup: false,
        }
    }
}

// create_config handles I/O
fn create_config(mut cfg: NdayConfig) -> Result<(), std::io::Error> {
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

    std::fs::create_dir_all(datapath_path)?;

    cfg.dir = datapath;
    cfg.tool = tool_input;
    cfg.setup = true;
    confy::store("nday", cfg).unwrap();

    Result::Ok(())
}

fn get_most_recent_before(dates: &Vec<NaiveDate>, before: &NaiveDate) -> Option<NaiveDate> {
    let mut recent: Option<NaiveDate> = None;
    for date in dates {
        if date < before {
            recent = Some(*date);
        }
    }
    recent
}

// fn get_kicked_from(contents: String) -> Vec<String> {
//     vec![]
// }

fn generate_new_note_text(date: &NaiveDate, kicked: &Vec<String>) -> String {
    let todays_text = date.format("%A %-e %B, %Y").to_string();
    let todo_text = format!("todo:\n{}", kicked.join("\n"));
    let done_text = "done:\n- ";
    let kicked_text = "kicked:\n- ";
    format!(
        "{}\n\n{}\n\n{}\n\n{}\n",
        todays_text, todo_text, done_text, kicked_text
    )
}

fn date_to_file_name(date: &NaiveDate) -> String {
    date.format("%0e%b%Y.txt").to_string()
}

fn file_name_to_date(file_name: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(file_name, "%0e%b%Y.txt")
}

// handles I/O
fn get_kicked_items(from_note: String) -> Vec<String> {
    let mut kicked_items: Vec<String> = Vec::new();
    let split = from_note.split('\n');
    let mut start_collecting = false;
    for s in split {
        if start_collecting && !s.is_empty() && !s.eq("- ") {
            kicked_items.push(s.to_string());
        }
        if s.eq("kicked:") {
            start_collecting = true;
        }
    }

    kicked_items
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Run first time setup
    #[clap(short, long)]
    setup: bool,
}

fn main() {
    let args: Args = Args::parse();
    let cfg_old: NdayConfig = confy::load("nday").expect("Could not locate config file");
    // if setup
    if args.setup || !cfg_old.setup {
        create_config(cfg_old).expect("Could not create config file");
    }
    // read config
    let cfg: NdayConfig = confy::load("nday").expect("Could not locate config file");

    // check for todays note
    let mut today_path = PathBuf::new();
    today_path.push(&cfg.dir);
    let local_dt: DateTime<Local> = Local::now();
    let local: NaiveDate = local_dt.naive_local().date();
    let local_file_name = date_to_file_name(&local);
    today_path.push(&local_file_name);

    if !today_path.exists() {
        // get kicked items
        let files_in_dir = read_dir(cfg.dir).expect("Could not read from notes folder");

        //map ReadDir to vector of NaiveDates
        // ReadDir -> DirFile -> file_name -> to_str -> to_date -> collect
        let file_dates: Vec<NaiveDate> = files_in_dir
            .filter_map(|f| f.ok())
            .map(|f| f.file_name())
            .filter_map(|f| file_name_to_date(f.to_str()?).ok())
            .collect();

        let kicked_items = match get_most_recent_before(&file_dates, &local) {
            Some(yesterday_date) => {
                let mut yesterday_file = File::open(date_to_file_name(&yesterday_date))
                    .expect("could not open last note file");
                let mut yesterday_file_contents = String::new();
                yesterday_file
                    .read_to_string(&mut yesterday_file_contents)
                    .expect("could not open last note file");

                get_kicked_items(yesterday_file_contents)
            }
            None => vec![],
        };

        // generate_new_note_text
        let new_note_text = generate_new_note_text(&local, &kicked_items);

        // create file
        let mut today_file = File::create(&today_path).expect("Could not create today's notes");
        // write contents
        today_file
            .write_all(new_note_text.as_bytes())
            .expect("could not write to today's notes");
    };

    // open todays note
    // let file_name_str = today_path
    //     .file_name()
    //     .expect("today's file is invalid")
    //     .to_str()
    //     .expect("today's file is invalid");
    // let note_date =
    //     NaiveDate::parse_from_str(file_name_str, "%0e%b%Y.txt").expect("Today's note is invalid");
    println!(
        "Using {} to open notes for today {}",
        cfg.tool,
        style(local.format("%A %B %-e")).cyan()
    );

    Command::new(cfg.tool)
        .arg(local_file_name.to_string())
        .status()
        .expect("Could not open today's notes using provided tool");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_most_recent_before() {
        let dates: Vec<chrono::NaiveDate> = vec![
            NaiveDate::from_ymd(2015, 3, 11),
            NaiveDate::from_ymd(2015, 3, 12),
            NaiveDate::from_ymd(2015, 3, 15),
            NaiveDate::from_ymd(2015, 3, 16),
            NaiveDate::from_ymd(2015, 3, 18),
            NaiveDate::from_ymd(2015, 4, 10),
        ];
        let empty_dates: Vec<chrono::NaiveDate> = vec![];

        let before: chrono::NaiveDate = NaiveDate::from_ymd(2015, 3, 18);
        let expected_date: chrono::NaiveDate = NaiveDate::from_ymd(2015, 3, 16);
        assert_eq!(get_most_recent_before(&dates, &before), Some(expected_date));

        let none_before: chrono::NaiveDate = NaiveDate::from_ymd(2014, 3, 18);
        assert_eq!(get_most_recent_before(&dates, &none_before), None);

        assert_eq!(get_most_recent_before(&empty_dates, &before), None);
    }

    #[test]
    fn test_generate_new_note_text() {
        let date: chrono::NaiveDate = NaiveDate::from_ymd(2015, 3, 18);
        let kicked = vec![
            String::from("- one"),
            String::from("- two"),
            String::from("- three"),
        ];

        let expected = String::from("Wednesday 18 March, 2015\n\ntodo:\n- one\n- two\n- three\n\ndone:\n- \n\nkicked:\n- \n");
        assert_eq!(generate_new_note_text(&date, &kicked), expected);
    }

    #[test]
    fn test_date_to_file_name() {
        let date: chrono::NaiveDate = NaiveDate::from_ymd(2015, 3, 18);

        assert_eq!(date_to_file_name(&date), String::from("18Mar2015.txt"));
    }

    #[test]
    fn test_file_name_to_date() {
        let date_string = String::from("18Mar2015.txt");

        assert_eq!(
            file_name_to_date(&date_string),
            Ok(NaiveDate::from_ymd(2015, 3, 18))
        );
    }

    #[test]
    fn test_get_kicked_items() {
        let file_string = String::from(
            "Thursday 16 June, 2022\ntodo:\n- \n\ndone:\n- \n\nkicked:\n- one\n- two\n- three\n",
        );

        let expected = vec![
            String::from("- one"),
            String::from("- two"),
            String::from("- three"),
        ];
        assert_eq!(get_kicked_items(file_string), expected);
    }

    #[test]
    fn test_get_kicked_items_empty() {
        let file_string =
            String::from("Thursday 16 June, 2022\ntodo:\n- \n\ndone:\n- \n\nkicked:\n- \n");

        let expected: Vec<String> = vec![];
        assert_eq!(get_kicked_items(file_string), expected);
    }
}
