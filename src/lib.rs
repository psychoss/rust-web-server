extern crate hyper;
extern crate time;
extern crate rustc_serialize;

#[macro_use]mod macros;

pub mod mime;
pub mod server;
pub mod static_handler;
// pub useuse sender::Sender;
pub use server::Flash;
