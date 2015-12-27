
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use mime::MediaType;
use hyper::server::response::Response;
use hyper::status::StatusCode;

pub struct Handler;

impl Handler {
    fn set_header(&self, res: &mut Response, p: &Path) {
        match MediaType.get_media_type(p.to_str().unwrap()) {
            Ok(v) => {
                res.headers_mut()
                   .set_raw("Content-Type", vec![v]);
            }
            Err(_) => {
                res.headers_mut()
                   .set_raw("Content-Type", vec![b"text/plain".to_vec()]);
            }
        }
    }
    pub fn handle(&self, p: &Path, mut res: Response) {
        if p.exists() && p.is_file() {
            match File::open(p) {
                Ok(ref mut file) => {
                    let mut v: Vec<u8> = Vec::new();
                    let _ = file.read_to_end(&mut v);
                    self.set_header(&mut res, p);
                    let _ = res.send(&v);
                }
                Err(_) => {
                    failed!(res, StatusCode::BadRequest);
                }
            }

        } else {
            failed!(res, StatusCode::BadRequest);
        }
        println!("{:?}", p);

    }
}
