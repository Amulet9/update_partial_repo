use ormlite::{sqlite::SqliteConnection, Connection};
use ormlite::model::*;
mod model;


use model::Test;
#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "sqlx=info");
    env_logger::init();
    let mut conn = SqliteConnection::connect("[path_to_db_file]").await.unwrap();
    sqlx::migrate!().run(&mut conn).await.unwrap();
    let new_instance = Test::builder().property(961).build().insert(&mut conn).await.unwrap();
    new_instance.update_partial().property(560).update(&mut conn).await.unwrap();
}



