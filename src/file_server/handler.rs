
use hyper::server::response::Response;
use hyper::server::request::Request;

use std::path::{PathBuf, Path};

pub struct Static {
    pub root: PathBuf,
}

impl Static {
    pub fn new<P:AsRef<Path>>(root:P)->Static{
        Static{
            root:root.as_ref().to_path_buf(),
        }
    }
    pub fn handle(&self,uri:&str){
println!("{}", self.root.to_str().unwrap());
    }
}
