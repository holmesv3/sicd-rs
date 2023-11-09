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
```
