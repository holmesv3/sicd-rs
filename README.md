# nitf-rs

[![crates.io](https://img.shields.io/crates/v/sicd-rs)](https://crates.io/crates/sicd-rs)
[![docs](https://img.shields.io/docsrs/sicd-rs)](https://docs.rs/sicd-rs/latest/sicd_rs/)

# sicd-rs

A rust SICD file interface

## Example

```rust
// Read sicd and print metadata
let file = std::path::Path::new("../example.nitf");
let sicd = sicd_rs::read_sicd(file);
println!("{:?}", sicd.meta);
```

If you have questions, would like to contribute, or would like to request
something be added, please create an issue.