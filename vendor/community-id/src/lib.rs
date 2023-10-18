//! [![github]](https://github.com/traceflight/rs-community-id)&ensp;[![crates-io]](https://crates.io/crates/community-id)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//!
//! <br>
//!
//! This package provides a Rust implementation of the open [Community ID](https://github.com/corelight/community-id-spec)
//! flow hashing standard.
//!
//! <br>
//!
//! # Community ID
//!
//! "Community ID" is a separate specification for generating a likely-unique identifier for a network connection proposed
//! by Corelight (the company behind Bro/Zeek). See [community-id-spec](https://github.com/corelight/community-id-spec)
//!
//! # Usage
//!
//! ```
//! use std::net::Ipv4Addr;
//! use community_id::calculate_community_id;
//!
//! let id = calculate_community_id(
//!     0,
//!     Ipv4Addr::new(1, 2, 3, 4).into(),
//!     Ipv4Addr::new(5, 6, 7, 8).into(),
//!     Some(1122),
//!     Some(3344),
//!     6,
//!     Default::default(),
//! );
//! assert_eq!("1:wCb3OG7yAFWelaUydu0D+125CLM=", id.unwrap());
//! ```

mod calc;
mod icmpv4;
mod icmpv6;
mod ipv4;
mod ipv6;

/// Default Padding
const PADDING: u8 = 0;

/// IP Protocol Number of ICMP
const IPPROTO_ICMP: u8 = 1;

/// IP Protocol Number or ICMPv6
const IPPROTO_ICMPV6: u8 = 58;

/// IP Protocol Number of SCTP
const IPPROTO_SCTP: u8 = 132;

/// IP Protocol Number of TCP
const IPPROTO_TCP: u8 = 6;

/// IP Protocol Number of UDP
const IPPROTO_UDP: u8 = 17;

pub use calc::calculate_community_id;
