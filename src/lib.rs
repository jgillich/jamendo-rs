/*!
jamendo-rs is a client for v3 of the Jamendo API. It exposes a client, whose
methods return an action that can be used to specify query parameters.

```
let client = jamendo::Client::new(jamendo::TEST_ID);
let albums = client.get_albums().limit(15).featured(true).unwrap();

for album in albums {
  println!("{}", album.name);
}
```
To create a new client, you need a client ID. The test ID changes regularly,
for productive usage you should get your own at https://developer.jamendo.com/v3.0/authentication.
*/

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
pub const TEST_ID: &'static str = "56d30c95";

pub use action::{Action, Resource, Query};
pub use response::{Response, Album, Artist, Track, User, UserAlbums, UserArtists, UserTracks, PartialAlbum, PartialArtist, PartialTrack};
pub use client::Client;
pub use error::{Error, ErrorKind};
pub use transport::Transport;

