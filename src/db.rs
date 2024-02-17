use crate::models::Package;
use dotenvy::dotenv;
use futures::TryStreamExt;
use futures_util::TryFutureExt;
use serde_json::to_vec;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::{SqliteConnection, SqlitePool};
use sqlx::Connection;
use sqlx::{Pool, Sqlite};
use std::env;
use std::process::Command;
use std::time::Instant;
use std::{str, string, vec::Vec};
use tokio;
const BIND_LIMIT: usize = 32766;

//https://stackoverflow.com/questions/27435839/how-to-list-active-connections-on-postgresql
pub async fn connect(db_url: &str) -> Result<sqlx::Pool<sqlx::Sqlite>, sqlx::Error> {
    dotenv().ok();
    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //let connection =    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
    // let conn = SqliteConnection::connect(&db_url)
    //     .await
    //     .unwrap_or_else(|_| panic!("Error connecting to {}", &db_url));
    //
    // let manager = ConnectionManager::<SqliteConnection>::new(&db_url);
    //
    // let pool1 = SqlitePool::connect(&db_url)
    //     .await
    //     .unwrap_or_else(|_| panic!("failed to connect to database: {}", &db_url));

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await?;
    Ok(pool)
}

pub async fn execute_get_all_pkg(conn: &Pool<Sqlite>) -> Result<Vec<Package>, sqlx::Error> {
    let mut pkg: Vec<Package> = Vec::new();
    let qstring: String =
        "SELECT * FROM packages WHERE name LIKE 'A%' ORDER BY name ASC".to_string();
    let pkg_query: Vec<Package> = sqlx::query_as!(
        Package,
        "SELECT * FROM packages WHERE name LIKE 'A%' ORDER BY name ASC"
    )
    .fetch_all(conn)
    .await?;
    Ok(pkg_query)
}

//https://stackoverflow.com/questions/68633531/imlementing-connection-pooling-in-a-rust-diesel-app-with-r2d2
// pub async fn fetch_packages(conn: &mut DbConn) -> Result<Vec<Package>, diesel::result::Error> {
//     Ok(packages.load::<Package>(conn)?)
// }
