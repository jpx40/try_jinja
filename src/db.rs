use diesel::pg;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::{connection, prelude::*};
use dotenvy::dotenv;
use r2d2;
use serde_json::to_vec;
use std::env;
use std::process::Command;
use std::time::Instant;
use std::{str, string, vec::Vec};
use tokio;

use crate::models::Package;
const BIND_LIMIT: usize = 32766;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

use crate::schema::packages::dsl::*;

//https://stackoverflow.com/questions/27435839/how-to-list-active-connections-on-postgresql
pub fn connect(db_url: &str) -> r2d2::Pool<diesel::r2d2::ConnectionManager<pg::PgConnection>> {
    dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //let connection =    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .max_size(4)
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn execute(
    conn: &mut PgConnection,
    pkg_list: Vec<Package>,
) -> Result<diesel::QueryResult<usize>, diesel::result::Error> {
    Ok(diesel::insert_into(packages)
        .values(&pkg_list)
        .execute(conn))
}

//https://stackoverflow.com/questions/68633531/imlementing-connection-pooling-in-a-rust-diesel-app-with-r2d2
pub async fn fetch_packages(conn: &mut DbConn) -> Result<Vec<Package>, diesel::result::Error> {
    Ok(packages.load::<Package>(conn)?)
}
