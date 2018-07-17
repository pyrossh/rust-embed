extern crate actix_web;
#[macro_use]
extern crate rust_embed;

use actix_web::{App, HttpRequest, HttpResponse, server};
use actix_web::http::Method;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
  match Asset::get(path) {
    Some(content) => HttpResponse::Ok().body(content),
    None => HttpResponse::NotFound().body("404 Not Found"),
  }
}

fn index(_req: HttpRequest) -> HttpResponse {
  handle_embedded_file("index.html")
}

fn dist(req: HttpRequest) -> HttpResponse {
  let path = &req.path()["/dist/".len()..];
  handle_embedded_file(path)
}

fn main() {
  server::new(|| {
    App::new().route("/", Method::GET, index).route(
      "/dist{_:.*}",
      Method::GET,
      dist,
    )
  }).bind("127.0.0.1:8000")
    .unwrap()
    .run();
}
