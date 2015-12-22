use hyper::Server as HttpServer;
use std::net::{ToSocketAddrs, SocketAddr};
use std::path::{PathBuf, Path};
use std::env;
use std::io::prelude::*;
use std::fs::File;

use hyper::error::Error;
use hyper::net::Fresh;
use hyper::server::request::Request;
use hyper::server::response::Response;
use hyper::uri::RequestUri::{AbsoluteUri, AbsolutePath};
use hyper::status::StatusCode;

pub struct Flash {
    address: Option<SocketAddr>,
    accept: Vec<&'static str>,
    pub root: PathBuf,
}

impl Flash {
    pub fn http<A: ToSocketAddrs>(self, addr: A) -> Result<(), Error> {
        let sock_addr = addr.to_socket_addrs()
                            .ok()
                            .and_then(|mut addrs| addrs.next())
                            .expect("Could not parse socket address.");
        let flash = try!(HttpServer::http(sock_addr));
        flash.handle(self);
        Ok(())
    }
    pub fn new() -> Flash {
        let s: Vec<&str> = "html ico js css".split(' ').collect();

        Flash {
            address: None,
            accept: s,
            root:env::current_dir().unwrap().as_path().join("static").to_path_buf()
        }
    }
    pub fn file_server(&self, uri: &String) {
        let file_path=self.root.as_path().join(uri);
        if file_path.exists()&& file_path.is_file(){
            println!("{:?}", file_path);            
        }
    }
}
impl ::hyper::server::Handler for Flash {
    fn handle(&self, req: Request, mut res: Response<Fresh>) {
        let (addr, method, headers, uri, _, reader) = req.deconstruct();
        // *res.status_mut() = status::InternalServerError;
        match uri {
            AbsoluteUri(ref url) => {
                println!("{:?}", url);
            }
            AbsolutePath(ref path) => {
                let mut p = path.clone();
                p.remove(0);
                if p.len() == 0 {
                    p.push_str("index.html");
                }
                let v: Vec<&str> = p.split(".").collect();
                if self.accept.contains(v.last().unwrap()) {
                    self.file_server(&p);
                    println!("{:?}", v);
                }
            }
            _ => {
                bad_request(res);
            }
        }


    }
}


fn bad_request(mut res: Response<Fresh>) {
    *res.status_mut() = StatusCode::BadRequest;

    if let Ok(res) = res.start() {
        let _ = res.end();
    }
}
