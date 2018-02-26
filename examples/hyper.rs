// extern crate futures;
// extern crate hyper;

// use futures::future::Future;

// use hyper::header::ContentLength;
// use hyper::server::{Http, Request, Response, Service};

// mod assets;

// struct HelloWorld;

// const PHRASE: &'static str = "Hello, World!";

// impl Service for HelloWorld {
//     // boilerplate hooking up hyper's server types
//     type Request = Request;
//     type Response = Response;
//     type Error = hyper::Error;
//     // The future representing the eventual Response your call will
//     // resolve to. This can change to whatever Future you need.
//     type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

//     fn call(&self, _req: Request) -> Self::Future {
//         // We're currently ignoring the Request
//         // And returning an 'ok' Future, which means it's ready
//         // immediately, and build a Response with the 'PHRASE' body.
//         Box::new(futures::future::ok(
//             Response::new()
//                 .with_header(ContentLength(PHRASE.len() as u64))
//                 .with_body(PHRASE),
//         ))
//     }
// }

// // fn handle_index(req: Request, res: Response) {
// //     match req.uri {
// //         AbsolutePath(ref path) => {
// //             println!("GET {:?}", &path);
// //             if &path[..] == "/" {
// //                 res.send(&assets::examples_public_index_html).unwrap();
// //             } else {
// //                 res.send(assets::get(&path[1..path.len()]).unwrap()).unwrap();
// //             }
// //         },
// //         _ => {
// //             return;
// //         }
// //     }
// // }

// fn main() {
//     println!("Server running on 127.0.0.1:3000");
//     let addr = "127.0.0.1:3000".parse().unwrap();
//     let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
//     server.run().unwrap();
// }

fn main() {
}