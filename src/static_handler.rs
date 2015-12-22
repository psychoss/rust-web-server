
use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf, Path};

pub struct Handler;

impl Handler {
    pub fn handle(p: &Path) -> Result<Vec<u8>, ()> {

        
        if p.exists() && p.is_file() {
            match File::open(p) {
                Ok(ref mut file) => {
                    let mut v: Vec<u8> = Vec::new();
                    let _ = file.read_to_end(&mut v);
                    Ok(v)
                }
                Err(_) => {
                    Err(())
                }
            }

        } else {
            Err(())
        }

    }
}

// use hyper::header::Headers;
// use hyper::server::response::Response;
//
// use std::env;

// header! { (ContentType, "Content-Type") => [String] }
//
// pub struct Sender {
//     accept: Vec<&'static str>,
//     pub root: PathBuf,
// }
//
// impl Sender {
//     pub fn send(&self,uri:&String,mut res: Response) -> Response {
//         res.headers_mut().set(ContentType("text/css".to_owned()));
//         res
//     }
//     pub fn get_file(&self, uri: &String) -> Option<Vec<u8>> {
//         let file_path = self.root.join(uri);
//         if file_path.exists() && file_path.is_file() {
//             match File::open(file_path) {
//                 Ok(ref mut f) => {
//                     let mut v: Vec<u8> = Vec::new();
//                     let _ = f.read_to_end(&mut v);
//                     Some(v)
//                 }
//                 Err(_) => {
//                     None
//                 }
//             }
//         } else {
//             None
//         }
//
//     }
//     pub fn new() -> Sender {
//         let s: Vec<&str> = "html ico js css".split(' ').collect();
//
//         Sender {
//             accept: s,
//             root: env::current_dir().unwrap().as_path().join("static"),
//         }
//     }
// }
