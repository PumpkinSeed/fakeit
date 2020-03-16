# fakeit

[![Latest Version](https://img.shields.io/crates/v/fakeit.svg)](https://crates.io/crates/fakeit)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.31.0+-green.svg)

Port of the famous [Go fakeit](https://github.com/brianvoe/gofakeit) library for Rust with more than 130 functions.

### Usage

- [Crates.io/fakeit](https://crates.io/crates/fakeit)
- [docs.rs](https://docs.rs/fakeit)

### Functions 
**(WIP, until checkout the docs.rs)**

- [address](#address-16-functions)
- [animal](#animal-6-functions)
- [beer](#beer-8-functions)
- bool
- color
- company
- contact
- currency
- datetime
- file
- generator
- hacker
- hipster
- image
- internet
- job
- language
- log-level
- name
- password
- payment
- person
- status code
- unique
- user agent
- vehicle
- words

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

##### animal (6 functions)

```rust
extern crate fakeit;

use fakeit::animal;

fn main() {
    let data = animal::pet_name(); // pet_name: Squeakers
    let data = animal::animal(); // animal: salmon
    let data = animal::type_of(); // type_of: fish
    let data = animal::farm(); // farm: Sheep
    let data = animal::cat(); // cat: Oriental Shorthair
    let data = animal::dog(); // dog: Rottweiler
}
```

##### beer (8 functions)

```rust
extern crate fakeit;

use fakeit::beer;

fn main() {
    let data = beer::name(); // name: Sierra Nevada Bigfoot Barleywine Style Ale
    let data = beer::style(); // style: Porter
    let data = beer::hop(); // hop: Equinox
    let data = beer::yeast(); // yeast: 1084 - Irish Ale
    let data = beer::malt(); // malt: Roasted barley
    let data = beer::ibu(); // ibu: 75 IBU
    let data = beer::alcohol(); // alcohol: 2.943696 %
    let data = beer::blg(); // blg: 7.4607124°Blg
}
```

### Todo:

Generators:
- generate.rs - resolve tags
- number
- string