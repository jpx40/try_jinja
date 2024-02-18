use crate::models::{Packages, Pkg};
use chrono::{NaiveDate, NaiveDateTime};
use dotenvy::dotenv;
use futures::TryStreamExt;
use futures_util::TryFutureExt;
use sea_query::{
    ColumnDef, Expr, Func, Iden, OnConflict, Order, OrderedStatement, Query, SqliteQueryBuilder,
    Table,
};
use sea_query_binder::SqlxBinder;
use serde_json::to_vec;
use serde_json::{json, Value as Json};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::{SqliteConnection, SqlitePool, SqliteRow};
use sqlx::{pool, Connection, Pool, Row, Sqlite};
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::time::Instant;
use std::{str, string, vec::Vec};
use tokio;
use uuid::Uuid;

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

pub async fn execute_get_all_pkg_precice(
    pool: &Pool<Sqlite>,
    search_str: String,
) -> Result<Vec<Pkg>, sqlx::Error> {
    let (sql, values) = Query::select()
        .columns([Packages::Name, Packages::Size, Packages::Version])
        .from(Packages::Table)
        .build_sqlx(SqliteQueryBuilder);
    let rows: Vec<Pkg> = sqlx::query_as_with::<_, Pkg, _>(&sql, values.clone())
        .fetch_all(pool)
        .await?;
    Ok(rows)
}
pub async fn execute_get_all_pkg(conn: &Pool<Sqlite>) -> Result<Vec<Pkg>, sqlx::Error> {
    let mut pkg: Vec<Pkg> = Vec::new();
    let mut qstring: String = String::new();

    // if search_str.is_empty() {
    //     qstring = "SELECT * FROM packages WHERE name ORDER BY name ASC".to_string();
    // } else {
    //     qstring = format!(
    //         "SELECT * FROM packages WHERE name LIKE '%*{}*%' ORDER BY name ASC",
    //         search_str
    //     );
    // }
    qstring = "SELECT * FROM packages WHERE name ORDER BY name ASC".to_string();

    println!("qstring: {}", qstring);
    //  let query = sqlx::query(qstring.to_string());
    let pkg_query: Vec<Pkg> = sqlx::query_as!(Pkg, "SELECT * FROM packages ORDER BY name ASC")
        .fetch_all(conn)
        .await?;
    if pkg_query.is_empty() {
        println!("no results found");
        panic!("something went wrong");
    }
    Ok(pkg_query)
}

//https://stackoverflow.com/questions/68633531/imlementing-connection-pooling-in-a-rust-diesel-app-with-r2d2
// pub async fn fetch_packages(conn: &mut DbConn) -> Result<Vec<Package>, diesel::result::Error> {
//     Ok(packages.load::<Package>(conn)?)
// }
