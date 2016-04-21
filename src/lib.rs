#![feature(custom_derive, plugin, question_mark)]
#![plugin(serde_macros)]

extern crate url;
extern crate serde;
extern crate serde_json;
extern crate hyper;

mod action;
mod client;
mod error;
mod response;
mod transport;

/// The test client id. Changes regularly, do not use it in a real application!
/// To get your own, go to https://developer.jamendo.com/v3.0/authentication
pub const TEST_ID: &'static str = "56d30c95";

pub use action::{Action, Resource, Query};
pub use response::{Response, Album, Artist, Track};
pub use client::Client;
pub use error::{Error, ErrorKind};
pub use transport::Transport;
