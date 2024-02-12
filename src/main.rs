mod api;
mod broadcast;
mod db;
mod models;
mod page;
mod schema;
use actix_files::Files;

use actix_web::{
    error, get, middleware, post,
    web::{self, service, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};

//use cached::Expiration;
use diesel::{pg, PgConnection};
use listenfd::ListenFd;

use r2d2;
use serde::de::value;
use std::convert::TryInto;
use std::time::{Duration, Instant};
use std::{
    env, io,
    path::{Path, PathBuf},
    sync::Arc,
};

use tokio::sync::Barrier;
use tokio::{fs, sync::RwLock};
//mod broadcast;
//use self::broadcast::Broadcaster;
use anyhow::{anyhow, Context};

#[derive(Clone)]
pub struct AppState {
    //pub db: sqlx::Pool<sqlx::Sqlite>,
    // pub db: Arc<SqlitePool>,
    pub db: db::DbPool,
}

fn main() {
    println!("Starting");
    let _ = start();
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    //     .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?);
    //
    //
    //
    //
    let _files_service = Files::new("/", "./static").path_filter(|path, _| {
        path.components().count() == 1
            && Path::new("./static")
                .join(path)
                .symlink_metadata()
                .map(|m| !m.file_type().is_symlink())
                .unwrap_or(false)
    });

    //  let expiry = cached::MyExpiry;
    // let eviction_listener = |key, _value, cause| {
    //     println!("Evicted key {key}. Cause: {cause:?}");
    // };

    let host = env::var("HOST").unwrap_or_else(|_| String::from("default_host"));
    let port = env::var("PORT").unwrap_or_else(|_| String::from("default_port"));
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
    let server_url = format!("{}:{}", host, port);

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        let pool = db::connect(&db_url);
        //        let pool = Arc::new(pool_conn);
        let app_state = AppState { db: pool.clone() };
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(
                Files::new("static", "./static")
                    .show_files_listing()
                    .prefer_utf8(true),
            )
            .wrap(middleware::Logger::default())
            //  .wrap(HtmxMiddleware)
            .configure(api::init)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };
    log::info!("Starting server at {}", server_url);
    println!("Starting server at {}", server_url);
    server.run().await?;
    Ok(())
}
