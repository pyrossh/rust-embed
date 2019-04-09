#![deny(warnings)]

#[macro_use]
extern crate rust_embed;
extern crate warp;

use std::borrow::Cow;
use warp::{filters::path::Tail, http::Response, Filter, Rejection, Reply};

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

fn main() {
  let index_hml = warp::get2().and(warp::path::end()).and_then(|| serve("index.html"));

  let dist = warp::path("dist").and(warp::path::tail()).and_then(|tail: Tail| serve(tail.as_str()));

  let routes = index_hml.or(dist);

  warp::serve(routes).run(([127, 0, 0, 1], 8080));
}

fn serve(path: &str) -> Result<impl Reply, Rejection> {
  let mime = mime_guess::guess_mime_type(path);

  let asset: Option<Cow<'static, [u8]>> = Asset::get(path);

  let file = asset.ok_or_else(|| warp::reject::not_found())?;

  Ok(Response::builder().header("content-type", mime.to_string()).body(file))
}
