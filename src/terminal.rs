use std::path::PathBuf;
use crate::file_work;
use crate::work::{Work, WorkParams};
use std::io::{self, Write};
use std::error::Error;

pub enum MainSelect{
    NewWork, EditWork, ExportWorks, PrintReadable, ImportFromJSON, ListFiles, Error
}

pub fn user_select(string_show: &str) -> Result<u8, Box<dyn Error>> {
    //!Prints string_show and returns user's input as on option in u8
    println!("{}", string_show);

    let mut select = String::new();

    io::stdout().flush()?; // Ensure prompt is displayed before user input
    io::stdin().read_line(&mut select)?;

    let select: u8 = select.trim().parse()?;
    Ok(select)
}


pub fn user_input<T>(string_show: &str) -> Result<T, Box<dyn Error>>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: Error + 'static{
    //!Prints string_show and returns parsed value in <T> type
    println!("{}", string_show);

    let mut input = String::new();

    io::stdout().flush()?; // Ensure prompt is displayed before user input
    io::stdin().read_line(&mut input)?;

    input.trim().parse::<T>()
        .map_err(|e| e.into()) // Convert parse error into a Box<dyn Error>
}

pub fn user_input_raw(string_show: &str) -> Result<String, Box<dyn Error>> {
    //!Prints string_show and returns raw user input
    println!("{}", string_show);

    let mut input = String::new();
    
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    
    Ok(input)
}

pub fn user_input_path_buf() -> PathBuf {
    //!Gets path from user input
    loop {
        match user_input_raw("Path to JSON file: ") {
            Ok(input) => {
                let trimmed_input = input.trim();
                if trimmed_input.is_empty() {
                    println!("Empty input. Please provide a valid path.");
                } else {
                    return PathBuf::from(trimmed_input);
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                eprintln!("Please try again.");
            }
        }
    }
}
pub fn select_print() -> MainSelect {
    //!Main select function.

    match user_select("Select:\n1) New work\n2) Edit work\n3) Export Year\n4) Print readable\n5) Import from JSON\n6) List saved files") {
        Ok(selection) => {
            match selection
            {
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
        _ => { MainSelect::Error }
    }
}

pub fn input_create_work_params() -> Result<Vec<WorkParams>, Box<dyn Error>> {
    //! Return all work parameters from the user input as a vector

    let name = user_input("Type work's name: ")?;
    let desc = user_input("Type work's description: ")?;
    let start_time = user_input("Type work's start_time: ")?;
    let end_time = user_input("Type work's end_time: ")?;

    Ok(vec![
        WorkParams::Name(name),
        WorkParams::Desc(desc),
        WorkParams::DateStart(start_time),
        WorkParams::DateEnd(end_time),
    ])
}

pub fn input_edit_work_params() -> Result<WorkParams, Box<dyn Error>> {
    //! Return a single work parameter (name, desc, date_start, date_end) from the user input

    let select = user_select("What param: \n1)Name\n2)Description\n3)Start time\n4)End time")?;

    let param = match select {
        1 | 2 => {
            // Handle name and description input
            user_input("New value:")?
        },
        3 | 4 => {
            // Handle date input differently
            user_input("New value:").map_err(|_| "Failed to parse date")?
        },
        _ => return Err("Invalid selection".into()),
    };

    Ok(match select {
        1 => WorkParams::Name(param),
        2 => WorkParams::Desc(param),
        3 => WorkParams::DateStart(param.parse().map_err(|_| "Failed to parse date")?),
        4 => WorkParams::DateEnd(param.parse().map_err(|_| "Failed to parse date")?),
        _ => return Err("Invalid selection".into()),
    })
}



pub fn export_works(work_vec: &Vec<Work>) {
    //!Exports work vector into json file
    let string = serde_json::to_string(work_vec).expect("Failed to export works");

    file_work::export_into_json(string);
}