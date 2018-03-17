#![feature(test, plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate rust_embed;
extern crate fern;
extern crate log;

use std::path::PathBuf;
use std::ffi::OsStr;
use std::io::Cursor;
use rocket::response;
use rocket::http::{ContentType, Status};
use rocket::State;
use rust_embed::*;

#[get("/")]
fn index<'r>(asset: State<Asset>) -> response::Result<'r> {
  asset("index.html".to_owned()).map_or_else(
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
fn dist<'r>(asset: State<Asset>, file: PathBuf) -> response::Result<'r> {
  let filename = file.display().to_string();
  let ext = file
    .as_path()
    .extension()
    .and_then(OsStr::to_str)
    .expect("Could not get file extension");
  let content_type = ContentType::from_extension(ext).expect("Could not get file content type");
  asset(filename.clone()).map_or_else(
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
  fern::Dispatch::new()
    .format(move |out, message, record| {
      out.finish(format_args!(
        "[{}][{}] {}",
        record.target(),
        record.level(),
        message
      ))
    })
    .level(log::LevelFilter::Info)
    .chain(std::io::stdout())
    .apply()
    .expect("Could not initialize logger");
  let asset = embed!("examples/public/".to_owned());
  rocket::ignite()
    .manage(asset)
    .mount("/", routes![index, dist])
    .launch();
}
