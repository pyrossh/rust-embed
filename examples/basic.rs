#![feature(attr_literals)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate rust_embed;
extern crate fern;

#[derive(RustEmbed)]
#[folder("examples/public/")]
struct Asset;

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
  let index_html = Asset::get("index.html").unwrap();
  println!("{:?}", std::str::from_utf8(&index_html));
}
