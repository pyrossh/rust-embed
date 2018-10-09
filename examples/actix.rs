extern crate actix_web;
#[macro_use]
extern crate rust_embed;
extern crate mime_guess;

use actix_web::http::Method;
use actix_web::{server, App, Body, HttpRequest, HttpResponse};
use mime_guess::guess_mime_type;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
  match Asset::get(path) {
    Some(content) => HttpResponse::Ok()
      .content_type(guess_mime_type(path).as_ref())
      .body(Body::from_slice(content.as_ref())),
    None => HttpResponse::NotFound().body("404 Not Found"),
  }
}

fn index(_req: HttpRequest) -> HttpResponse {
  handle_embedded_file("index.html")
}

fn dist(req: HttpRequest) -> HttpResponse {
  let path = &req.path()["/dist/".len()..]; // trim the preceding `/dist/` in path
  handle_embedded_file(path)
}

fn main() {
  server::new(|| App::new().route("/", Method::GET, index).route("/dist/{_:.*}", Method::GET, dist))
    .bind("127.0.0.1:8000")
    .unwrap()
    .run();
}
