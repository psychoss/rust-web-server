extern crate web;

use web::file_server::Static;
use web::server::Flash;

fn main() {
Flash::new().http("127.0.0.1:8989") ;
}
