pub struct Headers;

use hyper::net::Fresh;
use hyper::server::response::Response;
use std::path::Path;
use std::fs;

use time::{self, Timespec};
use std::os::unix::fs::MetadataExt;
impl Headers {
    pub fn set_headers(res: &mut Response<Fresh>, p: &Path) {
        res.headers_mut()
           .set_raw("Cache-Control", vec![b"max-age=31536000, public".to_vec()]);
        match fs::metadata(p) {
            Ok(ref v) => {
                let tm = &time::at(Timespec::new(v.mtime() as i64, v.mtime_nsec() as i32));
                let s = tm.to_utc().rfc822().to_string();
                res.headers_mut().set_raw("Date", vec![s.into_bytes().to_vec()]);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}
