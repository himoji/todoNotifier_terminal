use std::collections::HashMap;
use chrono::{Duration, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Year{
    pub weeks: Vec<Week>
}
impl Year {
    pub fn save(self) -> Vec<Week> {
        let week_hash: Vec<Week> = self.weeks;

        week_hash
    }


}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Week {
    pub week_number: i32,
    pub week_year: i32,

    pub monday:    Vec<Work>,
    pub tuesday:   Vec<Work>,
    pub wednesday: Vec<Work>,
    pub thursday:  Vec<Work>,
    pub friday:    Vec<Work>,
    pub saturday:  Vec<Work>,
    pub sunday:    Vec<Work>
}

impl Week {
    pub fn human_json(&self) -> String {
        let mut hash_json: HashMap<String, String> = HashMap::new();

        hash_json.insert("week_number".to_string(), self.week_number.to_string());
        hash_json.insert("week_year".to_string(), self.week_year.to_string());
        hash_json.insert("monday".to_string(), "sd".to_string());

        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Work {
    pub name: String,
    pub desc: String,

    //pub start_time: DateTime<Utc>,
    //pub duration: Duration,

    pub start_time: i64,
    pub duration: i64,
    pub end_time: i64, // to be calculated
}
impl Work {
    pub fn create_work(name: String, desc: String, duration_in_sec: i64) -> Work {
        Work{
            name,
            desc,
            start_time: Utc::now().timestamp(),
            duration: duration_in_sec,
            end_time: Utc::now().timestamp()+duration_in_sec //can it be =/= start_time?
        }
    }
    pub fn humanify_time(&self) -> HashMap<String, String> {
        let mut hash_json: HashMap<String, String> = HashMap::new();

        let duration = Duration::seconds(self.duration);
        let start_time = Utc::timestamp_opt(&Utc, self.start_time, 0).unwrap();
        let end_time = start_time + duration;

        hash_json.insert("name".to_string(), self.name.to_string());
        hash_json.insert("desc".to_string(), self.desc.to_string());
        hash_json.insert("start_time".to_string(), start_time.to_string());
        hash_json.insert("duration".to_string(), duration.to_string());
        hash_json.insert("end_time".to_string(), end_time.to_string());

        hash_json
    }
}

/*
Week {
    week_number,
    week_year,
    days {
        monday {works}
        monday {works}
        monday {works}
        monday {works}
        monday {works}
        monday {works}
    }
}
*/
