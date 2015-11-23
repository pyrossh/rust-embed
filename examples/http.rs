extern crate hyper;
use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

use hyper::uri::RequestUri::AbsolutePath;

mod assets;

fn hello(req: Request, res: Response<Fresh>) {
    res.send(&assets::index_html).unwrap();
}

fn main() {
  println!("Server running on 127.0.0.1:3000");
  Server::http("127.0.0.1:3000").unwrap().handle(hello);
}
