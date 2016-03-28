# jamendo-rs

A Jamendo API client for Rust.

[![Travis](https://img.shields.io/travis/jgillich/jamendo-rs.svg?style=flat-square)](https://travis-ci.org/jgillich/jamendo-rs)

### Example

```rust
let client = jamendo::Client::new(jamendo::TEST_ID);
let response = client.get_albums().limit(15).unwrap();

for album in response.results.iter() {
  println!("{}", album.name);
}
```
