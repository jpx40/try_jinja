mod api;
mod broadcast;
mod db;
mod models;
mod page;

use actix_files::Files;
use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{
    error, get, middleware, post,
    web::{self, service, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use minijinja::*;
use minijinja_autoreload::AutoReloader;
//use cached::Expiration;

use listenfd::ListenFd;

use once_cell::sync::Lazy;
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
    pub db: sqlx::Pool<sqlx::Sqlite>,
}

pub static ENV: Lazy<Environment<'static>> = Lazy::new(|| {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));
    env
});

fn main() {
    println!("Starting");
    let _ = start();
}

// fn init_template() -> Environment<_> {
//     let mut env = Environment::new();
//     let template_path = std::env::current_dir().unwrap().join("templates");
//
//     env.set_loader(move |name| {
//         let pieces = name.split('/');
//         let mut path = template_path.clone();
//         for piece in pieces {
//             if piece != "." && piece != ".." && !piece.contains('\\') {
//                 path.push(piece);
//             } else {
//                 return Ok(None);
//             }
//         }
//
//         match fs::read_to_string(path) {
//             Ok(result) => Ok(Some(result)),
//             Err(err) => {
//                 if err.kind() == std::io::ErrorKind::NotFound {
//                     Ok(None)
//                 } else {
//                     Err(
//                         Error::new(ErrorKind::TemplateNotFound, "failed to load template")
//                             .with_source(err),
//                     )
//                 }
//             }
//         }
//     });
//     env
// }
type Template = Environment<'static>;
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
    //
    // let _files_service = Files::new("/", "./static").path_filter(|path, _| {
    //     path.components().count() == 1
    //         && Path::new("./static")
    //             .join(path)
    //             .symlink_metadata()
    //             .map(|m| !m.file_type().is_symlink())
    //             .unwrap_or(false)
    // });

    //  let expiry = cached::MyExpiry;
    // let eviction_listener = |key, _value, cause| {
    //     println!("Evicted key {key}. Cause: {cause:?}");
    // };
    // let reloader = template_auto();
    // let env = reloader.acquire_env().unwrap();
    // let template = env.get_template("index.html").unwrap();

    // "Cache-Control", "max-age=86400"
    let host = env::var("HOST").unwrap_or_else(|_| String::from("default_host"));
    let port = env::var("PORT").unwrap_or_else(|_| String::from("default_port"));
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
    let server_url = format!("{}:{}", host, port);
    let pool = db::connect(&db_url)
        .await
        .unwrap_or_else(|_| panic!("Failed to connect to database."));

    let template: Template = make_env();
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        //        let pool = Arc::new(pool_conn);
        let app_state = AppState { db: pool.clone() };
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(template.clone()))
            .service(
                Files::new("static", "./static")
                    .show_files_listing()
                    .prefer_utf8(true),
            )
            .service(
                web::scope("/static")
                    .wrap(middleware::DefaultHeaders::new().add(("Cache-Control", "max-age=0")))
                    .service(
                        Files::new("/", "./static")
                            .path_filter(|path, _| {
                                path.components().count() == 1
                                    && Path::new("./static")
                                        .join(path)
                                        .symlink_metadata()
                                        .map(|m| !m.file_type().is_symlink())
                                        .unwrap_or(false)
                            })
                            .prefer_utf8(true),
                    ),
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

fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    // minijinja_embed::load_templates!(&mut env);

    // #[cfg(feature = "bundled")]
    // {
    //     minijinja_embed::load_templates!(&mut env);
    // }
    //
    #[cfg(not(feature = "bundled"))]
    {
        env.set_loader(minijinja::path_loader("templates"));
    }

    env
}

fn template_auto() -> AutoReloader {
    // If DISABLE_AUTORELOAD is set, then the path tracking is disabled.
    let disable_autoreload = env::var("DISABLE_AUTORELOAD").as_deref() == Ok("1");

    // If FAST_AUTORELOAD is set, then fast reloading is enabled.
    let fast_autoreload = env::var("FAST_AUTORELOAD").as_deref() == Ok("1");

    // The closure is invoked every time the environment is outdated to
    // recreate it.
    AutoReloader::new(move |notifier| {
        let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");
        let mut env = Environment::new();
        env.set_loader(path_loader(&template_path));

        if fast_autoreload {
            notifier.set_fast_reload(true);
        }

        // if watch_path is never called, no fs watcher is created
        if !disable_autoreload {
            notifier.watch_path(&template_path, true);
        }
        Ok(env)
    })
}
