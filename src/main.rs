mod file_work;
mod work;
mod terminal;

use chrono::TimeZone;
use work::Work;
fn main() {
    let MAX_SIZE: u8 = 10;
    let mut vector = Vec::with_capacity(MAX_SIZE as usize);

    'main_loop: loop {
        // select:
        //     new work, edit work, export
    }
    let work_1 = Work::from("Finish the H/w".to_string(), "Finish: \nGeometry, Algebra, English".to_string(), chrono::Utc.timestamp_opt(1123123, 0).unwrap().timestamp(), chrono::Utc.timestamp_opt(1123312, 0).unwrap().timestamp());
    println!("{}", work_1);
    println!("{}", work_1.to_json_string());
}