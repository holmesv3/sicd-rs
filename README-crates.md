# sicd-rs

A rust SICD file interface

## Example

```rust

// Read sicd and print metadata
use sicd_rs::read_sicd;
let sicd_meta = read_sicd(nitf_path);
println!("{:#?}", sicd_meta);
```
