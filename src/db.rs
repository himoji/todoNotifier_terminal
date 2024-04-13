use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client};
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::work::{Work, WorkParams};

#[derive(Deserialize, Serialize, Debug)]
struct UserData {
    id: Thing,
}

#[derive(Deserialize, Serialize, Debug)]
struct DBWork {
    name: String,
    desc: String,
    date_start: String,
    date_end: String,
}
impl DBWork{
    pub fn from_work(work: Work) -> DBWork {
        DBWork {
            name: work.name,
            desc: work.desc,
            date_start: work.date_start.to_string(),
            date_end: work.date_end.to_string(),
        }
    }
}


pub async fn add_work(db: &Surreal<Client>, name: String, desc: String, date_start: i64, date_end: i64) -> surrealdb::Result<()> {
    let created: Vec<UserData> = db.
        create("work")
        .content(DBWork::from_work(Work::from(name, desc, date_start, date_end))).await?;
    dbg!(created);
    Ok(())
}

pub async fn add_work_vec(db: &Surreal<Client>, params: Vec<WorkParams>) -> surrealdb::Result<()> {
    let created: Vec<UserData> = db.
        create("work")
        .content(DBWork::from_work(Work::from_vec(params).expect("Failed to create work from vec!"))).await?;
    dbg!(created);
    Ok(())
}

pub async fn select_all_works(db: &Surreal<Client>) -> surrealdb::Result<()> {
    let selected: Vec<UserData> = db.select("work").await?;
    dbg!(selected);
    
    Ok(())
}