#![feature(custom_derive, plugin, question_mark)]
#![plugin(serde_macros)]

extern crate url;
extern crate serde;
extern crate serde_json;
extern crate hyper;

mod actions;
mod models;
mod client;
mod error;
mod transport;

pub const TEST_ID: &'static str = "9d9f42e3";

pub use models::*;
pub use client::Client;
pub use error::{Error, ErrorKind};
pub use transport::Transport;
