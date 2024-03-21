mod file_work;
mod work;
mod terminal;

use chrono::TimeZone;
use work::Work;
use crate::terminal::MainSelect;

fn main() {
    let MAX_SIZE: u8 = 10;
    let mut vector = Vec::with_capacity(MAX_SIZE as usize);

    'main_loop: loop {

        match terminal::select_print() {
            MainSelect::NewWork => {
                //call work_create terminal
            }
            MainSelect::EditWork => {
                //call work_edit terminal 
            }
            MainSelect::ExportWorks => {
                //call export_year terminal 
            }
            MainSelect::Error => {
                println!("Something went wrong!");
                continue 'main_loop
            }
        }
        // select:
        //     new work, edit work, export
    }
    let work_1 = Work::from("Finish the H/w".to_string(), "Finish: \nGeometry, Algebra, English".to_string(), chrono::Utc.timestamp_opt(1123123, 0).unwrap().timestamp(), chrono::Utc.timestamp_opt(1123312, 0).unwrap().timestamp());
    println!("{}", work_1);
    println!("{}", work_1.to_json_string());
}