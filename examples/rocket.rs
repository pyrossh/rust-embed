#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;

use rocket::http::{ContentType, Status};
use rocket::response;
use rust_embed::RustEmbed;

use std::ffi::OsStr;
use std::io::Cursor;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

#[get("/")]
fn index<'r>() -> response::Result<'r> {
  Asset::get("index.html").map_or_else(
    || Err(Status::NotFound),
    |d| response::Response::build().header(ContentType::HTML).sized_body(Cursor::new(d)).ok(),
  )
}

#[get("/dist/<file..>")]
fn dist<'r>(file: PathBuf) -> response::Result<'r> {
  let filename = file.display().to_string();
  Asset::get(&filename).map_or_else(
    || Err(Status::NotFound),
    |d| {
      let ext = file
        .as_path()
        .extension()
        .and_then(OsStr::to_str)
        .ok_or_else(|| Status::new(400, "Could not get file extension"))?;
      let content_type = ContentType::from_extension(ext).ok_or_else(|| Status::new(400, "Could not get file content type"))?;
      response::Response::build().header(content_type).sized_body(Cursor::new(d)).ok()
    },
  )
}

fn main() {
  rocket::ignite().mount("/", routes![index, dist]).launch();
}
