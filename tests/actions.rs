extern crate jamendo;

use jamendo::Client;
#[test]
fn get_albums() {
    let client = Client::new(jamendo::TEST_ID);
    let response = client.get_albums().limit(15).unwrap();

    assert_eq!(15, response.headers.results_count);
    assert_eq!(15, response.results.len());
}

#[test]
fn get_album() {
    let client = Client::new(jamendo::TEST_ID);
    let response = client.get_albums().id(24).unwrap();

    assert_eq!("Premiers Jets", response.results.first().unwrap().name);
}
