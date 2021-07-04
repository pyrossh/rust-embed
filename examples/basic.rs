use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

fn main() {
  let index_html = Asset::get("index.html").unwrap();
  println!("{:?}", std::str::from_utf8(index_html.data.as_ref()));
}
