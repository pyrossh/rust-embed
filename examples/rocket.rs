#![feature(test, plugin, decl_macro, attr_literals)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate rust_embed;

use std::path::PathBuf;
use std::ffi::OsStr;
use std::io::Cursor;
use rocket::response;
use rocket::http::{ContentType, Status};

#[derive(RustEmbed)]
#[folder("examples/public/")]
struct Asset;

#[get("/")]
fn index<'r>() -> response::Result<'r> {
  Asset::get("index.html").map_or_else(
    || Err(Status::NotFound),
    |d| {
      response::Response::build()
        .header(ContentType::HTML)
        .sized_body(Cursor::new(d))
        .ok()
    },
  )
}

#[get("/dist/<file..>")]
fn dist<'r>(file: PathBuf) -> response::Result<'r> {
  let filename = file.display().to_string();
  let ext = file
    .as_path()
    .extension()
    .and_then(OsStr::to_str)
    .expect("Could not get file extension");
  let content_type = ContentType::from_extension(ext).expect("Could not get file content type");
  Asset::get(&filename.clone()).map_or_else(
    || Err(Status::NotFound),
    |d| {
      response::Response::build()
        .header(content_type)
        .sized_body(Cursor::new(d))
        .ok()
    },
  )
}

fn main() {
  rocket::ignite().mount("/", routes![index, dist]).launch();
}
