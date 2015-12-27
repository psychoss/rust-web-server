extern crate web;

use std::env;

use web::server::Flash;

fn main() {
    let mut root_path = env::current_dir().unwrap();
    root_path.push("static");
    let _ = Flash::new(root_path).http("127.0.0.1:8989");
}
