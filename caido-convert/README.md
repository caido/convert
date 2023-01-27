# 👋 Convert

Welcome to caido convert package! This repo contains the code we use to encode, hash, zip and much more. This library focus on providing a decent API for diverse encoding need in a web application context. It is Grapheme aware when needed and tries to provide predictable encoding/decoding behavior.

## Examples
### Base64
```rust
fn main() {
  let encoder = Base64Decode::new();
  let decoded = encoder.execute("Y2FpZG8=".as_bytes()).unwrap();
  println!("{}", String::from_utf8(decoded).unwrap()); // caido

  let encoder = Base64Encode::new();
  let encoded = encoder.execute("caido".as_bytes()).unwrap();
  println!("{}", String::from_utf8(encoded).unwrap()); // Y2FpZG8=
}
```
