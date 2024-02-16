use crate::page;

use actix_web_lab::{extract::Path, respond::Html};

use crate::broadcast::Broadcaster;
use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{
    error, get, middleware, post,
    web::{self, service},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
pub fn init(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("api").service(page::page);

    cfg.service(scope)
        .service(broadcast_msg)
        .service(event_stream)
        .service(page::index);
}

#[post("/broadcast/{msg}")]
async fn broadcast_msg(
    broadcaster: web::Data<Broadcaster>,
    Path((msg,)): Path<(String,)>,
) -> impl Responder {
    broadcaster.broadcast(&msg).await;
    HttpResponse::Ok().body("msg sent")
}

#[get("/events")]
async fn event_stream(broadcaster: web::Data<Broadcaster>) -> impl Responder {
    broadcaster.new_client().await
}
