#![feature(test, plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate rust_embed;

use std::path::PathBuf;
use std::io::Cursor;
use rocket::response;
use rocket::http::{ContentType, Status};
use rust_embed::*;

#[get("/")]
fn index<'r>() -> response::Result<'r> {
  let asset = embed!("examples/public".to_owned());
  asset("/index.html".to_owned()).map_or_else(
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
  let asset = embed!("examples/public/".to_owned());
  // let ext = Path::new(&filename).extension().and_then(OsStr::to_str).expect("Could not get file extension");
  // let content_type = ContentType::from_extension(ext).expect("Could not get file content type");
  asset(filename.clone()).map_or_else(
    || Err(Status::NotFound),
    |d| {
      response::Response::build()
        .header(ContentType::HTML)
        .sized_body(Cursor::new(d))
        .ok()
    },
  )
}

fn main() {
  rocket::ignite().mount("/", routes![index, dist]).launch();
}
