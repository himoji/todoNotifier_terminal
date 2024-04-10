use serde::{Deserialize, Serialize};
use surrealdb::{Surreal};
use surrealdb::engine::remote::ws::Ws;
use crate::work::Work;

#[derive(Deserialize, Serialize, Debug)]
struct UserData {
    user_id: i32,
    work_vec: Vec<Work>
}

#[tokio::main]
pub async fn main() -> surrealdb::Result<()>{
    let db = Surreal::new::<Ws>("0.0.0.0:8000").await?;

    db.use_ns("test").use_db("test").await?;

    let created: Vec<UserData> = db
        .create("userdata")
        .content(UserData {
            user_id: 1,
            work_vec: vec![Work::new(), Work::new()]
        })
        .await?;
    dbg!(created);

    let people: Vec<UserData> = db.select("userdata").await?;
    dbg!(people);

    let groups = db
        .query("SELECT * FROM userdata")
        .await?;
    dbg!(groups);

    Ok(())
    
}