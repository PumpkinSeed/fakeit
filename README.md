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
- [bool](#bool-1-functions)
- [color](#color-4-functions)
- [company](#company-4-functions)
- [contact](#contact-4-functions)
- [currency](#currency-4-functions)
- [datetime](#datetime-14-functions)
- [file](#file-2-functions)
- [generator](#generator-1-function)
- [hacker](#hacker-6-functions)
- [hipster](#hipster-3-functions)
- [image](#image-1-function)
- [internet](#internet-7-functions)
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
    let data = address::latitude_in_range(-30 as f64, 30 as f64); // latitude_in_range: -18.35571
    let data = address::longitude(); // longitude: 113.12952
    let data = address::longitude_in_range(-30 as f64, 30 as f64); // longitude_in_range: -16.484156
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

##### bool (1 functions)

```rust
extern crate fakeit;

use fakeit::bool;

fn main() {
    let data = bool::bool(); // true / false
}
```

##### color (4 functions)

```rust
extern crate fakeit;

use fakeit::beer;

fn main() {
    let data = color::full(); // full: LightYellow
    let data = color::hex(); // hex: #662461
    let data = color::safe(); // safe: black
    let data = color::rgb(); // rgb: [162, 98, 22]
}
```

##### company (4 functions)

```rust
extern crate fakeit;

use fakeit::company;

fn main() {
    let data = company::company(); // company: Rowe-Schoen
    let data = company::company_suffix(); // company_suffix: Inc
    let data = company::buzzword(); // buzzword: systemic
    let data = company::bs(); // bs: strategic
}
```

##### contact (4 functions)

```rust
extern crate fakeit;

use fakeit::contact;

fn main() {
    let data = contact::info(); // contect::Info
    let data = contact::phone(); // phone: 5173757868
    let data = contact::phone_formatted(); // phone_formatted: 382.450.6544
    let data = contact::email(); // email: benkuvalis@marks.org
}
```

##### currency (4 functions)

```rust
extern crate fakeit;

use fakeit::currency;

fn main() {
    let data = currency::compact(); // currency::Info
    let data = currency::short(); // short: SRD
    let data = currency::long(); // long: Burundi Franc
    let data = currency::price(1 as f64, 123 as f64); // price: 53.7
}
```

##### datetime (14 functions)

```rust
extern crate fakeit;

use fakeit::datetime;

fn main() {
    let data = datetime::month(); // month: 10
    let data = datetime::day(); // day: 10
    let data = datetime::week_day(); // week_day: 6
    let data = datetime::year(); // year: 1986
    let data = datetime::hour(); // hour: 10
    let data = datetime::minute(); // minute: 10
    let data = datetime::second(); // second: 10
    let data = datetime::nanosecond(); // nanosecond: 959678991
    let data = datetime::timezone(); // timezone: SA Pacific Standard Time
    let data = datetime::timezone_full(); // timezone_full: (UTC-04:00) Atlantic Time (Canada)
    let data = datetime::timezone_abv(); // timezone_abv: BST
    let data = datetime::timezone_offset(); // timezone_offset: 13
    let data = datetime::date_range("RFC3339", "RFC3339"); // date_range: 1979-01-06 23:03:10.918301212 UTC
    let data = datetime::date(); // date: 1979-01-06 23:03:10.918301212 UTC
}
```

##### file (2 functions)

```rust
extern crate fakeit;

use fakeit::file;

fn main() {
    let data = file::mime_type(); // mime_type: text/x-fortran
    let data = file::extension(); // extension: aspx
}
```

##### generator (1 function)

```rust
extern crate fakeit;

use fakeit::generator;

fn main() {
    let data = generator::generate("{person.first} {person.last} {contact.email} #?#?#?".to_string()); // data: Watson Connelly baileeprosacco@smitham.biz 6d0e0a
    // More details about this later
}
```

##### hacker (6 functions)

```rust
extern crate fakeit;

use fakeit::hacker;

fn main() {
    let data = hacker::phrase(); // phrase: parsing the sensor won't do anything, we need to bypass the open-source AGP sensor!
    let data = hacker::abbreviation(); // abbreviation: PCI
    let data = hacker::adjective(); // adjective: bluetooth
    let data = hacker::noun(); // noun: protocol
    let data = hacker::verb(); // verb: copy
    let data = hacker::ingverb(); // ingverb: transmitting
}
```

##### hipster (3 functions)

```rust
extern crate fakeit;

use fakeit::hipster;

fn main() {
    let data = hipster::word(); // word: fingerstache
    let data = hipster::sentence(12); // sentence: Itaque aliquid id ex repudiandae adipisci quibusdam excepturi deleniti qui alias animi.
    let data = hipster::paragraph(3, 4, 40, " ".to_string()); // paragraph: Voluptas minima delectus voluptatibus earum rerum accusamus consequatur sunt....
}
```

##### image (1 function)

```rust
extern crate fakeit;

use fakeit::image;

fn main() {
    let data = image::url(500, 500); // url: https://picsum.photos/500/500
}
```

##### internet (7 functions)

```rust
extern crate fakeit;

use fakeit::internet;

fn main() {
    let data = internet::domain_name(); // domain_name: productvisualize.net
    let data = internet::http_method(); // http_method: DELETE
    let data = internet::domain_suffix(); // domain_suffix: biz
    let data = internet::ipv4_address(); // ipv4_address: 196.140.182.201
    let data = internet::ipv6_address(); // ipv6_address: 2001:cafe:1248:1dc7:17dd:19f4:8798:621d
    let data = internet::mac_address(); // mac_address: 2D:3F:7E:5D:61:C1
    let data = internet::username(); // username: Nienow1881
}
```

### Todo:

Generators:
- generate.rs - resolve tags
- number
- string