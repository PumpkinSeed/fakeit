# fakeit

[![Latest Version](https://img.shields.io/crates/v/fakeit.svg)](https://crates.io/crates/fakeit)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.31.0+-green.svg)

Port of the famous [Go fakeit](https://github.com/brianvoe/gofakeit) library for Rust.

### Usage

- [Crates.io/fakeit](https://crates.io/crates/fakeit)
- [docs.rs](https://docs.rs/fakeit)

### Functions

- [address](#address)

##### address (16 functions)

```rust
extern crate fakeit;

use fakeit::address;

fn main() {
    let data = address::info(); // address::Info struct
    let data = address::street(); // street: 1128 South North Dakota borough
    let data = address::street_number(); // street_number: 3155
    let data = address::street_prefix(); // street_prefix: Port
    let data = address::street_name(); // street_name: Kansas
    let data = address::street_suffix(); // street_suffix: mouth
    let data = address::city(); // city: Schmelerburgh
    let data = address::state(); // state: Kentucky
    let data = address::state_abr(); // state_abr: WA
    let data = address::zip(); // zip: 75221
    let data = address::country(); // country: Romania
    let data = address::country_abr(); // country_abr: BI
    let data = address::latitude(); // latitude: -69.14192
    let data = address::latitude_in_range(); // latitude_in_range: -18.35571
    let data = address::longitude(); // longitude: 113.12952
    let data = address::longitude_in_range(); // longitude_in_range: -16.484156
}
```

### Todo:

Generators:
- generate.rs - resolve tags
- number
- string