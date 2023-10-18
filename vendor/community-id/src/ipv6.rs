use std::net::Ipv6Addr;

use anyhow::{anyhow, Result};
use base64::prelude::*;
use sha1::digest::Update;
use sha1::{Digest, Sha1};

use crate::{icmpv6, PADDING, IPPROTO_ICMP,IPPROTO_ICMPV6};

pub fn calculate_ipv6_community_id(
    seed: u16,
    src_ip: Ipv6Addr,
    dst_ip: Ipv6Addr,
    src_port: Option<u16>,
    dst_port: Option<u16>,
    ip_proto: u8,
    disable_base64: bool,
) -> Result<String> {
    let mut sip = src_ip.octets();
    let mut dip = dst_ip.octets();

    let mut sport = src_port.map(|p| p.to_be());
    let mut dport = dst_port.map(|p| p.to_be());

    let mut is_one_way = false;

    if src_port.is_some() && dst_port.is_some() {
        let tmp_src_port = src_port.unwrap();
        let tmp_dst_port = dst_port.unwrap();
        match ip_proto {
            IPPROTO_ICMPV6 => {
                let (src, dst, one_way) = icmpv6::get_port_equivalents(tmp_src_port, tmp_dst_port);
                is_one_way = one_way;
                sport = Some(src.to_be());
                dport = Some(dst.to_be());
            }
            IPPROTO_ICMP => return Err(anyhow!("icmpv4 can not over ipv6!")),
            _ => {}
        }
    }

    if !(is_one_way || src_ip < dst_ip || (src_ip == dst_ip && src_port < dst_port)) {
        std::mem::swap(&mut sip, &mut dip);
        std::mem::swap(&mut sport, &mut dport);
    }

    let hash = if src_port.is_some() && dst_port.is_some() {
        let ipv6 = Ipv6Data {
            seed: seed.to_be(),
            src_ip: sip,
            dst_ip: dip,
            proto: ip_proto,
            pad0: PADDING,
            src_port: sport.unwrap(),
            dst_port: dport.unwrap(),
        };
        Sha1::new().chain(ipv6).finalize()
    } else {
        let ipv6 = Ipv6DataWithoutPort {
            seed: seed.to_be(),
            src_ip: sip,
            dst_ip: dip,
            proto: ip_proto,
            pad0: PADDING,
        };
        Sha1::new().chain(ipv6).finalize()
    };

    match disable_base64 {
        false => Ok("1:".to_string() + &BASE64_STANDARD.encode(hash)),
        true => Ok("1:".to_string() + &hex::encode(hash)),
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
struct Ipv6Data {
    seed: u16,
    src_ip: [u8; 16],
    dst_ip: [u8; 16],
    proto: u8,
    pad0: u8,
    src_port: u16,
    dst_port: u16,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
struct Ipv6DataWithoutPort {
    seed: u16,
    src_ip: [u8; 16],
    dst_ip: [u8; 16],
    proto: u8,
    pad0: u8,
}

impl AsRef<[u8]> for Ipv6Data {
    fn as_ref(&self) -> &[u8] {
        let len = std::mem::size_of::<Ipv6Data>();
        let p = self as *const _ as *const _;
        unsafe { std::slice::from_raw_parts(p, len) }
    }
}

impl AsRef<[u8]> for Ipv6DataWithoutPort {
    fn as_ref(&self) -> &[u8] {
        let len = std::mem::size_of::<Ipv6DataWithoutPort>();
        let p = self as *const _ as *const _;
        unsafe { std::slice::from_raw_parts(p, len) }
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv6Addr;

    use super::calculate_ipv6_community_id;

    #[derive(Debug)]
    struct Ipv4Input {
        seed: u16,
        src_ip: Ipv6Addr,
        dst_ip: Ipv6Addr,
        src_port: Option<u16>,
        dst_port: Option<u16>,
        proto: u8,
    }

    impl From<(u16, &str, &str, Option<u16>, Option<u16>, u8)> for Ipv4Input {
        fn from(value: (u16, &str, &str, Option<u16>, Option<u16>, u8)) -> Self {
            Self {
                seed: value.0,
                src_ip: value.1.parse().unwrap(),
                dst_ip: value.2.parse().unwrap(),
                src_port: value.3,
                dst_port: value.4,
                proto: value.5,
            }
        }
    }
    fn test_baseline_ipv6_default_data() -> Vec<(Ipv4Input, String)> {
        let raw = vec![
            (
                (
                    0,
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    None,
                    None,
                    46,
                ),
                "1:wQ/2mwrnXP4lmJcHdGL5ePjR+e0=",
            ),
            (
                (
                    0,
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    None,
                    None,
                    46,
                ),
                "1:wQ/2mwrnXP4lmJcHdGL5ePjR+e0=",
            ),
            (
                (
                    0,
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    Some(128),
                    Some(0),
                    58,
                ),
                "1:0bf7hyMJUwt3fMED7z8LIfRpBeo=",
            ),
            (
                (
                    0,
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    Some(129),
                    Some(0),
                    58,
                ),
                "1:0bf7hyMJUwt3fMED7z8LIfRpBeo=",
            ),
            (
                (
                    0,
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    Some(146),
                    Some(0),
                    58,
                ),
                "1:fYC8+pz24E+EhANP1EZhpX0Dw10=",
            ),
        ];
        raw.into_iter()
            .map(|(r, exp)| (r.into(), exp.to_string()))
            .collect()
    }

    #[test]
    fn test_baseline_default() {
        for (input, exp) in test_baseline_ipv6_default_data() {
            let v = calculate_ipv6_community_id(
                input.seed,
                input.src_ip,
                input.dst_ip,
                input.src_port,
                input.dst_port,
                input.proto.into(),
                Default::default(),
            );
            assert_eq!(v.unwrap(), exp);
        }
    }

    fn test_baseline_ipv6_seed_data() -> Vec<(Ipv4Input, String)> {
        let raw = vec![
            (
                (
                    1,
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    None,
                    None,
                    46,
                ),
                "1:j6EyyfBzL4vWKHFXgUqb5Az3OxM=",
            ),
            (
                (
                    1,
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    None,
                    None,
                    46,
                ),
                "1:j6EyyfBzL4vWKHFXgUqb5Az3OxM=",
            ),
            (
                (
                    1,
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    Some(128),
                    Some(0),
                    58,
                ),
                "1:IO27GQzPuCtNnwFvjWALMHu5tJE=",
            ),
            (
                (
                    1,
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    Some(129),
                    Some(0),
                    58,
                ),
                "1:IO27GQzPuCtNnwFvjWALMHu5tJE=",
            ),
            (
                (
                    1,
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    Some(146),
                    Some(0),
                    58,
                ),
                "1:Cv4IKJe56BNkF0MBW2YxQ3Nwc3s=",
            ),
        ];
        raw.into_iter()
            .map(|(r, exp)| (r.into(), exp.to_string()))
            .collect()
    }

    #[test]
    fn test_baseline_seed_1() {
        for (input, exp) in test_baseline_ipv6_seed_data() {
            let v = calculate_ipv6_community_id(
                input.seed,
                input.src_ip,
                input.dst_ip,
                input.src_port,
                input.dst_port,
                input.proto.into(),
                Default::default(),
            );
            assert_eq!(v.unwrap(), exp);
        }
    }

    fn test_baseline_ipv6_disable_base64() -> Vec<(Ipv4Input, String)> {
        let raw = vec![
            (
                (
                    0,
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    None,
                    None,
                    46,
                ),
                "1:c10ff69b0ae75cfe259897077462f978f8d1f9ed",
            ),
            (
                (
                    0,
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    None,
                    None,
                    46,
                ),
                "1:c10ff69b0ae75cfe259897077462f978f8d1f9ed",
            ),
            (
                (
                    0,
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    Some(128),
                    Some(0),
                    58,
                ),
                "1:d1b7fb872309530b777cc103ef3f0b21f46905ea",
            ),
            (
                (
                    0,
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    Some(129),
                    Some(0),
                    58,
                ),
                "1:d1b7fb872309530b777cc103ef3f0b21f46905ea",
            ),
            (
                (
                    0,
                    "fe80:0001:0203:0405:0607:0809:0A0B:0C0D",
                    "fe80:1011:1213:1415:1617:1819:1A1B:1C1D",
                    Some(146),
                    Some(0),
                    58,
                ),
                "1:7d80bcfa9cf6e04f8484034fd44661a57d03c35d",
            ),
        ];
        raw.into_iter()
            .map(|(r, exp)| (r.into(), exp.to_string()))
            .collect()
    }

    #[test]
    fn test_baseline_disable_base64() {
        for (input, exp) in test_baseline_ipv6_disable_base64() {
            let v = calculate_ipv6_community_id(
                input.seed,
                input.src_ip,
                input.dst_ip,
                input.src_port,
                input.dst_port,
                input.proto.into(),
                true,
            );
            assert_eq!(v.unwrap(), exp);
        }
    }
}
