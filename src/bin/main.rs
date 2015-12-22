extern crate web;


use web::server::Flash;

fn main() {
    let _=Flash::new().http("127.0.0.1:8989");
}
