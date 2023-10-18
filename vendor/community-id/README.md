# rs-community-id

This package provides a Rust implementation of the open [Community ID](https://github.com/corelight/community-id-spec) flow hashing standard.

## Community ID

"Community ID" is a separate specification for generating a likely-unique identifier for a network connection proposed by Corelight (the company behind Bro/Zeek). See [community-id-spec](https://github.com/corelight/community-id-spec)

## Usage

```toml
# Cargo.toml
[dependencies]
community-id = "0.2"
```

```rust
use std::net::Ipv4Addr;
use community_id::calculate_community_id;

let id = calculate_community_id(
    0,
    Ipv4Addr::new(1, 2, 3, 4).into(),
    Ipv4Addr::new(5, 6, 7, 8).into(),
    Some(1122),
    Some(3344),
    6,
    Default::default(),
);
assert_eq!("1:wCb3OG7yAFWelaUydu0D+125CLM=", id.unwrap());

```


## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.