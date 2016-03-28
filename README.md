# jamendo-rs

A Jamendo API client for Rust.

### Example

```rust
let client = jamendo::Client::new(jamendo::TEST_ID);
let response = client.get_albums().limit(15).unwrap();

for album in response.results.iter() {
  println!("{}", album.name);
}
```
