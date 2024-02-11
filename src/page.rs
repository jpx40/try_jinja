use crate::db;
use crate::models::*;
use crate::schema::complain::content;
// use crate::schema::people::id;
// use crate::schema::users::id;
use crate::AppState;
use actix_files::NamedFile;
use actix_session::Session;
use actix_web::body;
use actix_web::http::header::CacheControl;
use actix_web::{
    dev, error, get, http::header::ContentType, http::StatusCode, middleware::ErrorHandlerResponse,
    web, web::Data, Error, HttpResponse, Responder, Result,
};
use diesel::pg;
use diesel::pg::PgConnection;
use html_node::html as rsx;
use hypertext::Renderable;
use itertools::Itertools;
use maud::{html, Markup, DOCTYPE};
use moka::{
    future::{Cache, FutureExt},
    notification::ListenerFuture,
    Expiry,
};

use r2d2::PooledConnection;
use serde::Deserialize;
use serde_json::value;
use std::string::String;
pub fn test() -> String {
    rsx! {}.to_string()
}

#[get("/")]
pub async fn index() -> Result<HttpResponse, Error> {
    let rendered = "Test";
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}

// #[derive(Deserialize)]
// struct Info {
//     id: u32,
// }

#[get("/page/pkgtable/{id}")]
pub async fn page(
    path: web::Path<(u32)>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let pool = &app_state.db;
    let mut conn: db::DbConn = pool.get().unwrap();
    let mut pkg_list: Vec<Package>;
    match db::fetch_packages(&mut conn).await {
        Ok(x) => {
            pkg_list = x;
        }
        Err(_x) => {
            pkg_list = Vec::new();
        }
    };

    let pkg = chunk_slice(pkg_list, 15).await;
    let num = path.into_inner();
    //
    let mut id_prev: u32;
    if num - 1 == 0 {
        id_prev = 1;
    } else {
        id_prev = num - 1;
    }

    let mut id_next = num + 1;

    let pkg_len = pkg.len() as u32;
    if num - 1 > pkg_len {
        id_next = pkg_len;
    }

    let url_prev = format!("api/page/pkgtable/{}", id_prev);
    let url_next = format!("api/page/pkgtable/{}", id_next);

    let rendered = package_table(pkg, num, url_prev, url_next).await;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}

#[get("/page/pkg")]
pub async fn page_pkg() -> Result<HttpResponse, Error> {
    let rendered = html! {};
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered.into_string()))
}
async fn package_table(
    pkg: Vec<Vec<Package>>,
    num: u32,
    url_prev: String,
    url_next: String,
) -> String {
    let html = html! {
    div #table  class="container mx-auto content-center justify-center" style="text-align: center;
    justify-content: center;  align-items: center;" {
          h1 class="text-2xl font-bold" {"Arch Linux Packages"}
              div class="container mx-auto content-center" style="text-align: center;
    justify-content: center; align-items: center;" {
              table class="table-auto mx-auto content-center" {
          thead { tr {
              th class="px-4 py-2" { "name" }
           th class="px-4 py-2" { "version" }
           th class="px-4 py-2" { "size" }
          }
          tbody {
          @for item in pkg[num as usize - 1 ].clone() {
          tr {
          td class="border px-4 py-2" { (item.name)}
          td class="border px-4 py-2" { (trim_version(item.size.to_string()))}
          td class="border px-4 py-2" { (item.version)}

          } }
               }}}
      }
    button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full" hx-get=(url_prev) hx-target="#table" hx-swap="innerHTML" { "Previous Page"}


      button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full" hx-get=(url_next) hx-target="#table" hx-swap="innerHTML"  { "Next Page"}
                 }
          };

    html.into_string()
}

pub async fn chunk_slice(mut pkg_list: Vec<Package>, mut chunk_size: u32) -> Vec<Vec<Package>> {
    let mut pkg_vec: Vec<Vec<Package>> = Vec::new();

    // pkg_vec = pkg_list
    //     .iter()
    //     .rev()
    //     .collect_vec()
    //     .chunks(chunk_size as usize)
    //     .into_iter()
    //     .map(|x| x.to_vec().into_iter().rev().collect_vec())
    //     .collect_vec();

    pkg_vec = pkg_list
        .into_iter()
        .rev()
        .collect_vec()
        .chunks(chunk_size as usize)
        .into_iter()
        .map(|x| x.to_vec().into_iter().rev().collect_vec())
        .collect_vec();

    // println!("{:?}", &pkg_list);
    pkg_vec
}

pub fn trim_version(version: String) -> String {
    let v: Vec<String> = version
        .split("+")
        .into_iter()
        .map(|x| x.to_string())
        .collect();

    let x: Vec<&str> = v[0].split("_").collect();

    x[0].to_string()
}
