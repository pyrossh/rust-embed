#[macro_use]
extern crate rouille;

use rouille::Response;

// TBD
fn main() {
  println!("Now listening on localhost:8000");
  rouille::start_server("localhost:8000", move |request| {
    {
      let response = rouille::match_assets(&request, ".");
      if response.is_success() {
        return response;
      }
    }
    Response::html(
      "404 error. Try <a href=\"/README.md\"`>README.md</a> or \
       <a href=\"/src/lib.rs\">src/lib.rs</a> for example.",
    ).with_status_code(404)
  });
}
