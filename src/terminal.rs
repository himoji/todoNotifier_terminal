use std::path::PathBuf;
use crate::file_work;
use crate::work::{Work, WorkParams};
use std::io::{self, Write};
use std::error::Error;

pub enum MainSelect{
    NewWork, EditWork, ExportWorks, PrintReadable, ImportFromJSON, ListFiles, Error
}

pub fn user_select(string_show: &str) -> u8 {
    //!Prints string_show and returns user's input as on option in u8
    let select = user_input_raw(string_show);

    let select: u8 = select.trim().parse().expect("Failed to parse the int!");
    select
}


pub fn user_input<T>(string_show: &str) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: Error{
    //!Prints string_show and returns parsed value in <T> type

    let input = user_input_raw(string_show);

    input.trim().parse::<T>().expect("Failed to parse to needed type") // Convert parse error into a Box<dyn Error>
}

pub fn user_input_raw(string_show: &str) -> String {
    //!Prints string_show and returns raw user input
    println!("{}", string_show);

    let mut input = String::new();
    loop {
        io::stdout().flush().expect("Failed to flush io::out"); // Ensure prompt is displayed before user input
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim whitespace from the input
        let input = input.trim();

        // Check if input is not empty
        if !input.is_empty() {
            break;
        }
        // If input is empty, prompt again
        println!("Please enter something:");
    }

    input
}

pub fn user_input_path_buf() -> PathBuf {
    //!Gets path from user input
    loop {
            let input = user_input_raw("Path to JSON file: ");
                if input.trim().is_empty() {
                    println!("Empty input. Please provide a valid path.");
                } else {
                    return PathBuf::from(input.trim());
                }
            }
        }
pub fn select_print() -> MainSelect {
    //!Main select function.

    match user_select("Select:\n1) New work\n2) Edit work\n3) Export Year\n4) Print readable\n5) Import from JSON\n6) List saved files") {
        1 => MainSelect::NewWork,
        2 => MainSelect::EditWork,
        3 => MainSelect::ExportWorks,
        4 => MainSelect::PrintReadable,
        5 => MainSelect::ImportFromJSON,
        6 => MainSelect::ListFiles,
        _ => {
            MainSelect::Error
        }
    }

}

pub fn input_create_work_params() -> Vec<WorkParams> {
    //! Return all work parameters from the user input as a vector

    let name = user_input("Type work's name: ");
    let desc = user_input("Type work's description: ");
    let start_time = user_input("Type work's start_time: ");
    let end_time = user_input("Type work's end_time: ");

    vec![
        WorkParams::Name(name),
        WorkParams::Desc(desc),
        WorkParams::DateStart(start_time),
        WorkParams::DateEnd(end_time),
    ]
}

pub fn input_edit_work_params() -> WorkParams {
    //! Return a single work parameter (name, desc, date_start, date_end) from the user input
    loop {
        let select = user_select("What param: \n1)Name\n2)Description\n3)Start time\n4)End time");

        match select {
            1=> WorkParams::Name(user_input::<String>("New value:")),
            2=> WorkParams::Desc(user_input::<String>("New value:")),
            3=> WorkParams::DateStart(user_input::<i64>("New value:")),
            4=> WorkParams::DateEnd(user_input::<i64>("New value:")),

            _ => {
                println!("Invalid selection. Please select a valid option.");
                continue; // Continue to the next iteration of the loop
            }
        };
    }
}



pub fn export_works(work_vec: &Vec<Work>) {
    //!Exports work vector into json file
    let string = serde_json::to_string(work_vec).expect("Failed to export works");

    file_work::export_into_json(string);
}