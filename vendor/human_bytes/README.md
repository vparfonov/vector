# human_bytes

> A Rust crate that converts bytes into human-readable values.

[![License](https://img.shields.io/crates/l/human_bytes?style=flat-square)](https://gitlab.com/forkbomb9/human_bytes-rs/-/blob/master/LICENSE)
[![Latest version](https://img.shields.io/crates/v/human_bytes?style=flat-square)](https://crates.io/crates/human_bytes)
[![Build status](https://img.shields.io/gitlab/pipeline/forkbomb9/human_bytes-rs?style=flat-square)]()

It can return either KB/MB/GB/TB or KiB/MiB/GiB/TiB via the `bibytes` feature,
which enables the [power of 2](https://en.wikipedia.org/wiki/Byte#Units_based_on_powers_of_2) unit system.

> (1 KB = 1024 B, 1 KiB = 1024 B, the only thing that changes is the suffix).

It supports from 0 bytes to several yottabytes (I cannot tell how many because I have to use `u128`s
to fit a single YB)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
human_bytes = "0.3"
# Or
human_bytes = { version = "0.3", features = ["bibytes"] }
```

And then

```rust
use human_bytes::human_bytes;

assert_eq!(human_bytes(563_200_u32), "550 KB".to_string());
// or
assert_eq!(human_bytes(563_200_u64 as f64), "550 KB".to_string());
// ________________________________/
// |
// | Needed only when you're using `u64` values,
// | because `f64` doesn't implement `std::convert::From<u64>`

// With the `bibytes` feature enabled:
assert_eq!(human_bytes(563_200_u32), "550 KiB".to_string());
```

The crate is dependency-free, but if you want an +/- 15% speed improvement, there's a `fast` feature
that uses [lexical](https://github.com/Alexhuszagh/rust-lexical) instead of `std::format!`
in the number-to-string conversion

```toml
[dependencies]
human_bytes = { version = "0.3", features = ["fast"] }
```

## About
The code is based on a PHP function I found [here](https://math.stackexchange.com/questions/247444/explain-convertion-algorithm-from-bytes-to-kb-mb-gb).

It is useful because you don't have to provide a prefix, it does it on its own.
It'll always return `1 MB` instead of `1000 KB`

It has some tests I wrote to check that the conversion is correct, and it returns decimals (e.g. `16.5 GB`)

## Changelog
Check the [CHANGELOG.md](./CHANGELOG.md)

## License
[BSD 2-clause](./LICENSE) (c) 2020 Namkhai B.
