use crate::work::{Work, WorkParams};

pub enum MainSelect{
    NewWork, EditWork, ExportWorks, PrintReadable,Error
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


pub fn select_print() -> MainSelect {
    match user_select("Select:\n1)New work\n2)Edit work\n3)Export Year\n4)Print readable") {
        1 => {MainSelect::NewWork},
        2 => {MainSelect::EditWork},
        3 => {MainSelect::ExportWorks},
        4 => {MainSelect::PrintReadable},
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

pub fn export_works(work_vec: &Vec<Work>) -> String {
    let mut string = String::new();

    for work in work_vec{
        string.push_str(&work.to_json_string());
    }

    string
}