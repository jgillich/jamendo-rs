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
    let albums = client.get_albums().album_id(24).unwrap();
    let album = albums.first().unwrap();
    assert_eq!("Premiers Jets", album.name);
    assert_eq!(24, album.id);
}

#[test]
fn get_users_albums() {
    let client = Client::new(jamendo::TEST_ID);
    let users = client.get_users_albums().user_id(972174).unwrap();
    assert_eq!(10, users.first().unwrap().albums.len());
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
    let artists = client.get_artists().artist_id(5).unwrap();
    assert_eq!("Both", artists.first().unwrap().name);
}


#[test]
fn get_users_artists() {
    let client = Client::new(jamendo::TEST_ID);
    let users = client.get_users_artists().user_id(972174).unwrap();
    assert_eq!(10, users.first().unwrap().artists.len());
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
    let tracks = client.get_tracks().track_id(1123578).unwrap();
    assert_eq!("Easier to run (Linkin Park cover)", tracks.first().unwrap().name);
}


#[test]
fn get_users_tracks() {
    let client = Client::new(jamendo::TEST_ID);
    let users = client.get_users_tracks().user_id(972174).unwrap();
    assert_eq!(10, users.first().unwrap().tracks.len());
}