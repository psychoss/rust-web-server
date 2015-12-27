use headers::Headers;
use hyper::Server as HttpServer;
use hyper::error::Error as HttpError;
use hyper::method::Method;
use hyper::net::Fresh;
use hyper::server::request::Request;
use hyper::server::response::Response;
use hyper::status::StatusCode;
use hyper::uri::RequestUri::{AbsoluteUri, AbsolutePath};

use static_handler::Handler as StaticHandler;
use std::io::Read;
use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::str;
use std::time::Duration;

use util;

pub struct Flash {
    root: PathBuf,
}

impl Flash {
    pub fn http<A: ToSocketAddrs>(self, addr: A) -> Result<(), HttpError> {
        let sock_addr = addr.to_socket_addrs()
                            .ok()
                            .and_then(|mut addrs| addrs.next())
                            .expect("Could not parse socket address.");
        let mut flash = try!(HttpServer::http(sock_addr));
        flash.keep_alive(Some(Duration::new(1, 0)));
        let _ = flash.handle(self);
        Ok(())
    }
    pub fn new(p: PathBuf) -> Flash {

        Flash { root: p }
    }
    pub fn dispatch(&self, uri: &String, req: &mut Request, mut res: Response) {
        match &req.method {
            &Method::Get => {
                let path = parse_uri(uri);
                let mut file_path = self.root.clone();
                file_path.push(&path);
                match util::get_extension(&path) {
                    Some(_) => {
                        Headers::set_headers(&mut res, &file_path);
                    }
                    None => {}
                }
                StaticHandler.handle(&file_path, res);

            }
            &Method::Post => {
                let mut v: Vec<u8> = Vec::new();
                let _ = req.read_to_end(&mut v);
                // match read_json(&v) {
                //     Ok(v) => {
                //         println!("Parse Json");
                //         let j = v.as_object().unwrap().get("item").unwrap();
                //         println!("{:?}", j);
                //     }
                //     Err(_) => {
                //         println!("Parse Json Error");
                //         failed!(res, StatusCode::BadRequest);

                //     }
                // }
                println!("{:?}", v);
            }
            _ => {
                failed!(res, StatusCode::BadRequest);

            }
        }
    }
}

impl ::hyper::server::Handler for Flash {
    fn handle<'a>(&self, mut req: Request, mut res: Response<'a, Fresh>) {
        let uri = req.uri.clone();
        match uri {
            AbsoluteUri(ref url) => {
                println!("{:?}", url);
            }
            AbsolutePath(ref path) => {
                self.dispatch(path, &mut req, res);
            }
            _ => {
                failed!(res, StatusCode::BadRequest);

            }
        }
    }
}



fn parse_uri(uri: &String) -> String {

    // println!("{:?}", Url::parse(uri).unwrap());
    let mut path = uri.clone();
    path.remove(0);
    if path.len() == 0 {
        path.push_str("index.html");
    }
    path
}
