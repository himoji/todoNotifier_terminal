mod file_work;
mod work;

use chrono::TimeZone;
use work::Work;
fn main() {
    let work_1 = Work::from("Finish the H/w".to_string(), "Finish: \nGeometry, Algebra, English".to_string(), chrono::Utc.timestamp_opt(1123123, 0).unwrap().timestamp(), chrono::Utc.timestamp_opt(1123312, 0).unwrap().timestamp());
    println!("{}", work_1.to_string());
    println!("{}", work_1.to_json_string());
}