use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::work::{Work, WorkParams};

#[derive(Deserialize, Serialize, Debug)]
struct UserData {
    id: Thing,
}

#[allow(dead_code)]
pub async fn add_work(
    db: &Surreal<Client>,
    name: String,
    desc: String,
    date_start: i64,
    date_end: i64,
) -> surrealdb::Result<()> {
    let created: Vec<UserData> = db
        .create("work")
        .content(Work::from(name, desc, date_start, date_end))
        .await?;
    dbg!(created);
    Ok(())
}

#[allow(dead_code)]
pub async fn add_work_params_vec(
    db: &Surreal<Client>,
    params: Vec<WorkParams>,
) -> surrealdb::Result<()> {
    let created: Vec<UserData> = db
        .create("work")
        .content(Work::from_vec(params).expect("Failed to create work from vec!"))
        .await?;
    dbg!(created);
    Ok(())
}

pub async fn add_filter_works_vec(db: &Surreal<Client>, vec: Vec<Work>) -> surrealdb::Result<()> {
    let mut resp = db.query("select * from work").await?;
    let old_vec: Vec<Work> = resp.take(0)?;

    let a = Work::remove_duplicates(old_vec, vec);
    dbg!(&a);

    add_works_vec(db, a).await?;

    Ok(())
}

pub async fn add_works_vec(db: &Surreal<Client>, vec: Vec<Work>) -> surrealdb::Result<()> {
    for work in vec {
        let created: Vec<UserData> = db.create("work").content(work).await?;
        dbg!(created);
    }
    Ok(())
}

#[allow(dead_code)]
pub async fn select_all_works(db: &Surreal<Client>) -> surrealdb::Result<()> {
    let selected: Vec<UserData> = db.select("work").await?;
    dbg!(selected);

    Ok(())
}
