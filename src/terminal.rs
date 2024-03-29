use std::path::PathBuf;
use crate::file_work;
use crate::work::{Work, WorkParams};

pub enum MainSelect{
    NewWork, EditWork, ExportWorks, PrintReadable, ImportFromJSON, Error
}

pub fn user_select(string_show: &str) -> u8 {
    println!("{}", string_show);

    let mut select = Default::default();

    std::io::stdin().read_line(&mut select).expect("Failed to get user selected");
    
    let select: u8 = select.trim().parse().unwrap();
    select
    
}

pub fn user_input<T: std::str::FromStr>(string_show: &str) -> T {
    println!("{}", string_show);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to get user input");

    match input.trim().parse::<T>() {
        Ok(value) => value,
        Err(_) => panic!("Failed to parse input"),
    }
}

pub fn user_input_raw(string_show: &str) -> String {
    println!("{}", string_show);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to get user input");
    
    input
}

pub fn user_input_path_buf() -> std::path::PathBuf {
    let input = user_input_raw("Path to JSON file: ");
    std::path::PathBuf::from(input.trim())
}
pub fn select_print() -> MainSelect {
    match user_select("Select:\n1)New work\n2)Edit work\n3)Export Year\n4)Print readable\n5)Import from JSON") {
        1 => {MainSelect::NewWork},
        2 => {MainSelect::EditWork},
        3 => {MainSelect::ExportWorks},
        4 => {MainSelect::PrintReadable},
        5 => {MainSelect::ImportFromJSON},
        _ => {MainSelect::Error}
    }
}

pub fn input_create_work_params() -> Vec<WorkParams> {
    let name: String = user_input("Type work's name: ");
    let desc: String = user_input("Type work's description: ");
    let start_time: i64 = user_input("Type work's start_time: ");
    let end_time: i64 = user_input("Type work's end_time: ");

    let vec = vec![
        WorkParams::Name(name),
        WorkParams::Desc(desc),
        WorkParams::DateStart(start_time),
        WorkParams::DateEnd(end_time),
    ];


    vec
}

pub fn input_edit_work_params() -> WorkParams {
    let select = user_select("What param: \n1)Name\n2)Description\n3)Start time\n4)End time");
    match select {
        1 => {
            let param = user_input("New value:");
            WorkParams::Name(param)
        },
        2 => {
            let param = user_input("New value:");
            WorkParams::Desc(param)
        },
        3 => {
            let param = user_input("New value:");
            WorkParams::DateStart(param)
        },
        4 => {
            let param = user_input("New value:");
            WorkParams::DateEnd(param)
        },

        _ => {panic!("Failed to get input edit params")}
    }
}

pub fn export_works(work_vec: &Vec<Work>, path_buf: PathBuf) {
    let string = serde_json::to_string(work_vec).expect("Failed to export works");
    
    file_work::write_into_file(path_buf.as_path(), string)
}