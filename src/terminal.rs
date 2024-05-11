use std::error::Error;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::{file_work, time_work};
use crate::work::{Work, WorkParams};

pub enum MainSelect {
    NewWork,
    EditWork,
    ExportWorks,
    PrintReadable,
    ImportFromJSON,
    ListFiles,
    Error,
}

pub fn user_select<T>(str_vec: Vec<T>) -> u8
where
    T: AsRef<Path>,
{
    //!Prints string_show and returns user's input as on option in u8

    for (i, string) in str_vec.iter().enumerate() {
        println!("#{}) {}", i + 1, string.as_ref().to_string_lossy())
    }

    let select = user_input_raw("");

    let select: u8 = select.trim().parse().expect("Failed to parse the int!");
    select
}

pub fn user_input<T>(string_show: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: Error,
{
    //!Prints string_show and returns parsed value in <T> type

    let input = user_input_raw(string_show);

    input
        .trim()
        .parse::<T>()
        .expect("Failed to parse to needed type") // Convert parse error into a Box<dyn Error>
}

pub fn user_input_raw(string_show: &str) -> String {
    //!Prints string_show and returns raw user input
    println!("{}", string_show);

    let mut input = String::new();
    loop {
        io::stdout().flush().expect("Failed to flush io::out"); // Ensure prompt is displayed before user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

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

    match user_select(vec![
        "New work",
        "Edit work",
        "Export Year",
        "Print readable",
        "Import from JSON",
        "List saved files",
    ]) {
        1 => MainSelect::NewWork,
        2 => MainSelect::EditWork,
        3 => MainSelect::ExportWorks,
        4 => MainSelect::PrintReadable,
        5 => MainSelect::ImportFromJSON,
        6 => MainSelect::ListFiles,
        _ => MainSelect::Error,
    }
}

pub fn input_create_work_params() -> Vec<WorkParams> {
    //! Return all work parameters from the user input as a vector

    let name = user_input("Type work's name: ");
    let desc = user_input("Type work's description: ");
    let start_time: i64 = time_work::time_parser(user_input(
        "Type work's start time (e.g. 1241252516, now, in 1h, in 1m, in 1h;1m): ",
    ));

    let end_time: i64 = time_work::time_parser(user_input(
        "Type work's end time (e.g. 1241252516, now, in 1h, in 1m, in 1h;1m): ",
    ));

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
        let select = user_select(vec!["Name", "Description", "Start time", "End time"]);

        match select {
            1 => return WorkParams::Name(user_input::<String>("New value:")),
            2 => return WorkParams::Desc(user_input::<String>("New value:")),
            3 => return WorkParams::DateStart(user_input::<i64>("New value:")),
            4 => return WorkParams::DateEnd(user_input::<i64>("New value:")),

            _ => {
                println!("Invalid selection. Please select a valid option.");
                continue; // Continue to the next iteration of the loop
            }
        };
    }
}

pub fn export_works(work_vec: Vec<Work>) {
    //!Exports work vector into json file
    let mut old_works = Work::from_vec_string(file_work::read_file(
        file_work::get_export_json_loc().as_path(),
    ));
    old_works.extend(work_vec);

    let string =
        serde_json::to_string(&old_works).expect("Failed to convert all works into string");

    file_work::export_into_json(string);
}

pub fn export_filter_works(work_vec: Vec<Work>) {
    //!Exports work vector into json file
    let mut old_works = Work::from_vec_string(file_work::read_file(
        file_work::get_export_json_loc().as_path(),
    ));
    old_works.extend(work_vec);

    let string = serde_json::to_string(&Work::filter_duplicates(old_works))
        .expect("Failed to convert all works into string");

    file_work::export_into_json(string);
}

pub fn import_from_default(path: &Path, vector: &mut Vec<Work>) {
    if path.file_name().expect("Failed to find any saved files") == "saved.json" {
        let add_works = Work::from_vec_string(file_work::read_file(path));
        vector.extend(add_works);
        println!("Imported successfully");
    }
}

pub trait PathBufDisplay {
    fn to_string_lossy(&self) -> String;
}

impl PathBufDisplay for PathBuf {
    #[warn(unconditional_recursion)]
    fn to_string_lossy(&self) -> String {
        self.to_string_lossy().to_string()
    }
}
