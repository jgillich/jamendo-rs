extern crate jamendo;

use jamendo::Client;

#[test]
fn get_albums() {
    let client = Client::new(jamendo::TEST_ID);
    let albums = client.get_albums().limit(15).unwrap();
    assert_eq!(15, albums.len());
}

#[test]
fn get_album() {
    let client = Client::new(jamendo::TEST_ID);
    let albums = client.get_albums().id(24).unwrap();
    assert_eq!("Premiers Jets", albums.first().unwrap().name);
}

#[test]
fn get_artists() {
    let client = Client::new(jamendo::TEST_ID);
    let artists = client.get_artists().limit(15).unwrap();
    assert_eq!(15, artists.len());
}

#[test]
fn get_artist() {
    let client = Client::new(jamendo::TEST_ID);
    let artists = client.get_artists().id(5).unwrap();
    assert_eq!("Both", artists.first().unwrap().name);
}

#[test]
fn get_tracks() {
    let client = Client::new(jamendo::TEST_ID);
    let tracks = client.get_tracks().limit(15).unwrap();
    assert_eq!(15, tracks.len());
}

#[test]
fn get_track() {
    let client = Client::new(jamendo::TEST_ID);
    let tracks = client.get_tracks().id(1123578).unwrap();
    assert_eq!("Easier to run (Linkin Park cover)", tracks.first().unwrap().name);
}
