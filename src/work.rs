/*
File defining a struct and impl for it.
Explanation:
    name: Stands for task's/work's name
    desc: Stands for task's/work's description
    date_start: Stands for task's/work's start DateTime in seconds
    date_end: Stands for task's/work's end DateTime in seconds
*/

use std::time::Duration;
use chrono::TimeZone;

pub enum WorkParams {
    Name(String), 
    Desc(String),
    DateStart(i64),
    DateEnd(i64)
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Work {
    name: String,
    desc: String,
    date_start: i64,
    date_end: i64
}

impl Work{
    pub fn new() -> Work {
        //!Creates default empty work
        Work{
            name: "".to_string(),
            desc: "".to_string(),
            date_start: 0,
            date_end: 0,
        }
    }
    pub fn from(name: String, desc: String, date_start: i64, date_end: i64) -> Work {
        //!Creates work with specified params
        Work{name, desc, date_start, date_end}
    }
    pub fn from_vec(vec: Vec<WorkParams>) -> Work {
        //!Parse from vector of works aka Vec< Work >
        //!Iteration through all items in vector
        let mut name= String::new();
        let mut desc= String::new();
        let mut date_start = 0;
        let mut date_end= 0;
        for i in vec {
            match i {
                WorkParams::Name(s) => {name = s;}
                WorkParams::Desc(s) => {desc = s;}
                WorkParams::DateStart(i) => {date_start = i;}
                WorkParams::DateEnd(i) => {date_end = i;}
            }
        }
        Work{name,desc,date_start, date_end}
    }
    pub fn from_vec_string(string: String) -> Vec<Work> {
        //!Parse from String Vec
        serde_json::from_str(&string).map_err(|e| print!("Failed to convert into work: {e}")).unwrap()
    }
    pub fn to_json_string(&self) -> String {
        //!Converts work to json, return as String
        serde_json::to_string(&self).map_err(|e| print!("Failed to convert into string: {e}")).unwrap()
    }
    
    pub fn edit(&mut self, param: WorkParams) {
        //!Edits specified param
        match param{
            WorkParams::Name(new_val) => {
                self.name = new_val;
            }
            WorkParams::Desc(new_val) => {
                self.desc = new_val;
            }
            WorkParams::DateStart(new_val) => {
                self.date_start= new_val;
            }
            WorkParams::DateEnd(new_val)=> {
                self.date_end = new_val;
            }
        }
    }
    
    pub fn get_time_progress(&self) -> u8 {
        //!Returns the percent of time passed
        if chrono::Utc::now().timestamp() <= self.date_start {0}
        else if chrono::Utc::now().timestamp() >= self.date_end {100}
        else {
            return ((chrono::Utc::now().timestamp() - self.date_start) / self.date_end - self.date_start) as u8;
        }
    }
}

impl std::fmt::Display for Work {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Name: {}\nDesc: {}\nStart: {}\nDuration: {}\nEnd: {}\nProgress by time passed: {}%\n", self.name, self.desc, chrono::Utc.timestamp_opt(self.date_start, 0).unwrap(), humantime::format_duration(Duration::from_secs((self.date_end - self.date_start).unsigned_abs())), chrono::Utc.timestamp_opt(self.date_end, 0).unwrap(), self.get_time_progress())
    }
}

impl std::str::FromStr for Work {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}