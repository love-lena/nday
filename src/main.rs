use chrono::prelude::*;
use chrono::Local;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
//use chrono::format;

static TEMPLATE_TEXT: &str =
    "DATE\n\nTo-do today:\n- \n\nDone today:\n- \n\nKicked to tomorrow:\n- \n";

fn main() {
    let local: Date<Local> = Local::today();
    //println!("{}", local.format("%b%-e").to_string());

    let path_str = local.format("%b%-e.txt").to_string();
    let path = Path::new(&path_str);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(_) => match File::create(&path) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display, why)
            }
            Ok(mut new_file) => {
                match new_file.write_all(TEMPLATE_TEXT.as_bytes()) {
                    Err(why) => panic!("couldn't write to {}: {}", display, why),
                    Ok(_) => println!("created today's notes {}", display),
                };
                new_file
            }
        },

        Ok(file) => {
            println!("opened today's notes {}", display);
            file
        }
    };

    // Open the path in read-only mode, returns `io::Result<File>`
    // let mut ready_file = match File::open(&path) {
    //     Err(_why) => match File::create(&path) {

    //         Err(why) => panic!("couldn't create {}: {}", display, why),

    //         Ok(file) => match file.write_all(TEMPLATE_TEXT.as_bytes()) {
    //                 Err(why) => panic!("couldn't write to {}: {}", display, why),
    //                 Ok(_) => {println!("successfully wrote to {}", display);file;},
    //             }
    //     },
    //     Ok(existing_file) => existing_file,
    // };

    // // Open a file in write-only mode, returns `io::Result<File>`
    // let mut file = match File::create(&path) {
    //     Err(why) => panic!("couldn't create {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // // Open a file in write-only mode, returns `io::Result<File>`
    // let file_option = OpenOptions::new().read(true).write(true).create(true).open(&path);
    // let file = match file_option {
    //     Err(why) => panic!("couldn't create {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    // match file.write_all(TEMPLATE_TEXT.as_bytes()) {
    //     Err(why) => panic!("couldn't write to {}: {}", display, why),
    //     Ok(_) => println!("successfully wrote to {}", display),
    // }
}
