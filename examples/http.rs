extern crate hyper;
use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::uri::RequestUri::AbsolutePath;

mod assets;

fn handle_index(req: Request, res: Response) {
    match req.uri {
        AbsolutePath(ref path) => {
            println!("GET {:?}", &path);
            if &path[..] == "/" {
                res.send(&assets::examples_public_index_html).unwrap();
            } else {
                res.send(assets::get(&path[1..path.len()]).unwrap()).unwrap();
            }
        },
        _ => {
            return;
        }
    }
}

fn main() {
  println!("Server running on 127.0.0.1:3000");
  Server::http("127.0.0.1:3000").unwrap().handle(handle_index).unwrap();
}
