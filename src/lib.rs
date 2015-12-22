extern crate hyper;

pub mod file_server;
pub mod server;

pub use file_server::Static;
pub use server::Flash;
