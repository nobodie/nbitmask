
# nbitmask

todo :
- full documentation


---

[![CI Checks](https://github.com/nobodie/nbitmask/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/nobodie/nbitmask/actions/workflows/rust.yml)

This crate provides an easy to use api to create n sized bitmasks. Bitwise operators are available and you can define the underlying container adapted to your environment. 

Supported container types : 
* u8
* u16
* u32
* u64
* u128

```
[dependencies]
nbitmask = "1.0.0"
```

---

### Api sample

```rust
use nbitmask::BitMask;

let ones: BitMask<u64> = BitMask::ones(7);
let mut mask: BitMask<u64> = BitMask::zeros(4);

mask.set(0, true).unwrap();
// Will display an out of bound error as bit 4 doesn't exist in mask
mask.set(4, true).unwrap_or_else(|err| println!("{}", err));

mask &= &ones;
println!("mask size didn't change : {}", mask.size());

let mask_copy = mask.clone();

mask <<= 1;
assert_eq!(mask.to_string(), "0100".to_string());

mask >>= 1;
assert_eq!(mask.to_string(), mask_copy.to_string());
assert_eq!(mask, mask_copy);
```

---

### License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
