/*
File defining a struct and impl for it.
Explanation:
    name: Stands for task's/work's name
    desc: Stands for task's/work's description
    date_start: Stands for task's/work's start DateTime in seconds
    date_end: Stands for task's/work's end DateTime in seconds
*/

use chrono::TimeZone;
pub enum WorkParam {
    Name(String), 
    Desc(String),
    DateStart(i64),
    DateEnd(i64)
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Work{
    name: String,
    desc: String,
    date_start: i64,
    date_end: i64
}

impl Work{
    pub fn new() -> Work {
        Work{
            name: "".to_string(),
            desc: "".to_string(),
            date_start: 0,
            date_end: 0,
        }
    }
    pub fn from(name: String, desc: String, date_start: i64, date_end: i64) -> Work {
        Work{name, desc, date_start, date_end}
    }

    pub fn from_string(string: String) -> Work{
        serde_json::from_str(&string).unwrap()
    }
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self).map_err(|e| print!("Failed to convert into string: {e}")).unwrap()
    }
    
    pub fn edit(&mut self, param: WorkParam) {
        match param{
            WorkParam::Name(new_val) => {
                self.name = new_val;
            }
            WorkParam::Desc(new_val) => {
                self.desc = new_val;
            }
            WorkParam::DateStart(new_val) => {
                self.date_start= new_val;
            }
            WorkParam::DateEnd(new_val)=> {
                self.date_end = new_val;
            }
        }
    }
}

impl std::fmt::Display for Work {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Name: {}\nDesc: {}\nStart: {}\nDuration: ~{} minutes\nEnd: {}\n", self.name, self.desc, chrono::Utc.timestamp_opt(self.date_start, 0).unwrap(), (self.date_end - self.date_start)/60, chrono::Utc.timestamp_opt(self.date_end, 0).unwrap())
    }
}