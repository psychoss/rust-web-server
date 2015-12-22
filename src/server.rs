
use hyper::Server as HttpServer;
use hyper::error::Error as HttpError;
use hyper::method::Method;
use hyper::net::Fresh;
use hyper::server::request::Request;
use hyper::server::response::Response;
use hyper::status::StatusCode;
use hyper::uri::RequestUri::{AbsoluteUri, AbsolutePath};
use std::os::unix::fs::MetadataExt;
use std::fs::{self, Metadata};
use time::{self, Timespec};
use std::env;
use std::net::{ToSocketAddrs, SocketAddr};
use std::path::PathBuf;
use std::time::Duration;
use static_handler::Handler as StaticHandler;
use std::io::Read;
use std::any::Any;
use std::marker;

pub struct Flash {
    address: Option<SocketAddr>,
    root: PathBuf,
}

impl Flash {
    pub fn http<A: ToSocketAddrs>(self, addr: A) -> Result<(), HttpError> {
        let sock_addr = addr.to_socket_addrs()
                            .ok()
                            .and_then(|mut addrs| addrs.next())
                            .expect("Could not parse socket address.");
        let mut flash = try!(HttpServer::http(sock_addr));
        // Set the keep_alive
        flash.keep_alive(Some(Duration::new(1, 0)));
        flash.handle(self);
        Ok(())
    }
    pub fn new() -> Flash {
        let mut root_path = env::current_dir().unwrap();
        root_path.push("static");
        Flash {
            address: None,
            root: root_path,
        }
    }
    pub fn dispatch(&self,  req:&mut Request,uri: &String, method: &Method, mut res: Response) {
        match method {
            &Method::Get => {
                let path = parse_uri(uri);
                let mut file_path = self.root.clone();
                file_path.push(&path);
                match get_extension(&path) {
                    Some(v) => {
                        res=set_header(res,&file_path);
                    }
                    None => {}
                }

                match StaticHandler::handle(&file_path) {
                    Ok(ref v) => {
                        res.send(v);
                    }
                    _ => {
                        bad_request(res);
                    }
                }
            }
            &Method::Post => {
                let mut v:Vec<u8>=Vec::new();
                req.read_to_end(&mut v);
                println!("{:?}",v );
            },
            _ => {
                bad_request(res);
            }
        }
    }


}
impl ::hyper::server::Handler for Flash {
    fn handle<'a>(&self, mut req: Request, mut res: Response<'a,Fresh>) {
        //let (_, method, _, uri, _, _) = req.deconstruct();
        let method=req.method.clone();
        let uri=req.uri.clone();
        match uri {
            AbsoluteUri(ref url) => {
                println!("{:?}", url);
            }
            AbsolutePath(ref path) => {
                self.dispatch(&mut req,path, &method, res);
            }
            _ => {
                bad_request(res);
            }
        }
    }
}

fn set_header<'a,'b>(mut res: Response<'a,Fresh>, file_path: &PathBuf) -> Response<'a,Fresh> {
    res.headers_mut()
       .set_raw("Cache-Control", vec![b"max-age=31536000, public".to_vec()]);
       let path=&file_path.to_string_lossy().into_owned();
    match fs::metadata(path) {
        Ok(ref v) => {
            let tm = &time::at(Timespec::new(v.mtime() as i64, v.mtime_nsec() as i32));
            let s = tm.to_utc().rfc822().to_string();
            res.headers_mut().set_raw("Date", vec![s.into_bytes().to_vec()]);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
    let v = get_extension(path).unwrap();
    if v == "css" {
        res.headers_mut().set_raw("content-type", vec![b"text/css".to_vec()]);
    } else if v == "js" {
        res.headers_mut()
           .set_raw("content-type", vec![b"application/javascript".to_vec()]);
    }else if v=="ico"{
        res.headers_mut()
           .set_raw("content-type", vec![b"image/x-icon".to_vec()]);
    }
    res
}
fn get_extension(uri: &String) -> Option<&str> {
    uri.split(".").last()
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

fn bad_request(mut res: Response<Fresh>) {
    *res.status_mut() = StatusCode::BadRequest;

    if let Ok(res) = res.start() {
        let _ = res.end();
    }
}
