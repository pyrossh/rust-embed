use axum::{
  body::{boxed, Full},
  handler::Handler,
  http::{header, StatusCode, Uri},
  response::Response,
  routing::Router,
};
use rust_embed::RustEmbed;
use std::net::SocketAddr;

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "examples/axum-spa/assets/"]
struct Assets;

#[tokio::main]
async fn main() {
  let app = Router::new().fallback(static_handler.into_service());

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  println!("listening on {}", addr);
  axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn static_handler(uri: Uri) -> Response {
  let path = uri.path().trim_start_matches('/');

  if path.is_empty() || path == INDEX_HTML {
    return index_html().await;
  }

  match Assets::get(path) {
    Some(content) => {
      let body = boxed(Full::from(content.data));
      let mime = mime_guess::from_path(path).first_or_octet_stream();

      Response::builder().header(header::CONTENT_TYPE, mime.as_ref()).body(body).unwrap()
    }
    None => {
      if path.contains('.') {
        return not_found().await;
      }

      index_html().await
    }
  }
}

async fn index_html() -> Response {
  match Assets::get(INDEX_HTML) {
    Some(content) => {
      let body = boxed(Full::from(content.data));

      Response::builder().header(header::CONTENT_TYPE, "text/html").body(body).unwrap()
    }
    None => not_found().await,
  }
}

async fn not_found() -> Response {
  Response::builder().status(StatusCode::NOT_FOUND).body(boxed(Full::from("404"))).unwrap()
}
