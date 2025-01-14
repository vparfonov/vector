use std::net::Ipv4Addr;

use anyhow::{anyhow, Result};
use base64::prelude::*;
use sha1::digest::Update;
use sha1::{Digest, Sha1};

use crate::{icmpv4, IPPROTO_ICMP, IPPROTO_ICMPV6, PADDING};

pub fn calculate_ipv4_community_id(
    seed: u16,
    src_ip: Ipv4Addr,
    dst_ip: Ipv4Addr,
    src_port: Option<u16>,
    dst_port: Option<u16>,
    ip_proto: u8,
    disable_base64: bool,
) -> Result<String> {
    let mut sip = <Ipv4Addr as Into<u32>>::into(src_ip).to_be();
    let mut dip = <Ipv4Addr as Into<u32>>::into(dst_ip).to_be();

    let mut is_one_way = false;

    let mut sport = src_port;
    let mut dport = dst_port;

    match ip_proto {
        IPPROTO_ICMP => {
            let (src, dst, one_way) = icmpv4::get_port_equivalents(
                src_port.unwrap_or_default(),
                dst_port.unwrap_or_default(),
            );
            is_one_way = one_way;
            sport = Some(src);
            dport = Some(dst);
        }
        IPPROTO_ICMPV6 => return Err(anyhow!("icmpv6 can not over ipv4!")),
        _ => {}
    }

    if !(is_one_way || src_ip < dst_ip || (src_ip == dst_ip && sport < dport)) {
        std::mem::swap(&mut sip, &mut dip);
        std::mem::swap(&mut sport, &mut dport);
    }

    sport = sport.map(|p| p.to_be());
    dport = dport.map(|p| p.to_be());

    let hash = if sport.is_some() && dport.is_some() {
        let ipv4 = Ipv4Data {
            seed: seed.to_be(),
            src_ip: sip,
            dst_ip: dip,
            proto: ip_proto,
            pad0: PADDING,
            src_port: sport.unwrap(),
            dst_port: dport.unwrap(),
        };
        Sha1::new().chain(ipv4).finalize()
    } else {
        let ipv4 = Ipv4DataWithoutPort {
            seed: seed.to_be(),
            src_ip: sip,
            dst_ip: dip,
            proto: ip_proto,
            pad0: PADDING,
        };
        Sha1::new().chain(ipv4).finalize()
    };

    match disable_base64 {
        false => Ok("1:".to_string() + &BASE64_STANDARD.encode(hash)),
        true => Ok("1:".to_string() + &hex::encode(hash)),
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
struct Ipv4Data {
    seed: u16,
    src_ip: u32,
    dst_ip: u32,
    proto: u8,
    pad0: u8,
    src_port: u16,
    dst_port: u16,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
struct Ipv4DataWithoutPort {
    seed: u16,
    src_ip: u32,
    dst_ip: u32,
    proto: u8,
    pad0: u8,
}

impl AsRef<[u8]> for Ipv4Data {
    fn as_ref(&self) -> &[u8] {
        let len = std::mem::size_of::<Ipv4Data>();
        let p = self as *const _ as *const _;
        unsafe { std::slice::from_raw_parts(p, len) }
    }
}

impl AsRef<[u8]> for Ipv4DataWithoutPort {
    fn as_ref(&self) -> &[u8] {
        let len = std::mem::size_of::<Ipv4DataWithoutPort>();
        let p = self as *const _ as *const _;
        unsafe { std::slice::from_raw_parts(p, len) }
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::calculate_ipv4_community_id;

    #[derive(Debug)]
    struct Ipv4Input {
        seed: u16,
        src_ip: Ipv4Addr,
        dst_ip: Ipv4Addr,
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
    fn test_baseline_ipv4_default_data() -> Vec<(Ipv4Input, String)> {
        let raw = vec![
            (
                (0, "0.0.0.0", "0.0.0.0", Some(23376), Some(443), 6),
                "1:EWt4TLjkII9rdzFzQrCecjyvdNs=",
            ),
            (
                (0, "1.2.3.4", "5.6.7.8", Some(1122), Some(3344), 6),
                "1:wCb3OG7yAFWelaUydu0D+125CLM=",
            ),
            (
                (0, "5.6.7.8", "1.2.3.4", Some(3344), Some(1122), 6),
                "1:wCb3OG7yAFWelaUydu0D+125CLM=",
            ),
            (
                (0, "1.2.3.4", "5.6.7.8", Some(1122), Some(3344), 17),
                "1:0Mu9InQx6z4ZiCZM/7HXi2WMhOg=",
            ),
            (
                (0, "5.6.7.8", "1.2.3.4", Some(3344), Some(1122), 17),
                "1:0Mu9InQx6z4ZiCZM/7HXi2WMhOg=",
            ),
            (
                (0, "1.2.3.4", "5.6.7.8", Some(1122), Some(3344), 132),
                "1:EKt4MsxuyaE6mL+hmrEkQ9csDD8=",
            ),
            (
                (0, "5.6.7.8", "1.2.3.4", Some(3344), Some(1122), 132),
                "1:EKt4MsxuyaE6mL+hmrEkQ9csDD8=",
            ),
            (
                (0, "1.2.3.4", "5.6.7.8", Some(8), Some(0), 1),
                "1:crodRHL2FEsHjbv3UkRrfbs4bZ0=",
            ),
            (
                (0, "5.6.7.8", "1.2.3.4", Some(0), Some(0), 1),
                "1:crodRHL2FEsHjbv3UkRrfbs4bZ0=",
            ),
            (
                (0, "1.2.3.4", "5.6.7.8", Some(11), Some(0), 1),
                "1:f/YiSyWqczrTgfUCZlBUnvHRcPk=",
            ),
            (
                (0, "1.2.3.4", "5.6.7.8", None, None, 46),
                "1:ikv3kmf89luf73WPz1jOs49S768=",
            ),
            (
                (0, "5.6.7.8", "1.2.3.4", None, None, 46),
                "1:ikv3kmf89luf73WPz1jOs49S768=",
            ),
            (
                (0, "100.66.124.24", "172.31.98.44", None, None, 47),
                "1:Uo11LCySQ1S0c9jtHZVIb4Pm/2k=",
            ),
        ];
        raw.into_iter()
            .map(|(r, exp)| (r.into(), exp.to_string()))
            .collect()
    }

    #[test]
    fn test_baseline_default() {
        for (input, exp) in test_baseline_ipv4_default_data() {
            let v = calculate_ipv4_community_id(
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

    fn test_baseline_ipv4_seed_data() -> Vec<(Ipv4Input, String)> {
        let raw = vec![
            (
                (1, "1.2.3.4", "5.6.7.8", Some(1122), Some(3344), 6),
                "1:HhA1B+6CoLbiKPEs5nhNYN4XWfk=",
            ),
            (
                (1, "5.6.7.8", "1.2.3.4", Some(3344), Some(1122), 6),
                "1:HhA1B+6CoLbiKPEs5nhNYN4XWfk=",
            ),
            (
                (1, "1.2.3.4", "5.6.7.8", Some(1122), Some(3344), 17),
                "1:OShq+iKDAMVouh/4bMxB9Sz4amw=",
            ),
            (
                (1, "5.6.7.8", "1.2.3.4", Some(3344), Some(1122), 17),
                "1:OShq+iKDAMVouh/4bMxB9Sz4amw=",
            ),
            (
                (1, "1.2.3.4", "5.6.7.8", Some(1122), Some(3344), 132),
                "1:uitchpn5MMGAQKBJh7bIr/bAr7s=",
            ),
            (
                (1, "5.6.7.8", "1.2.3.4", Some(3344), Some(1122), 132),
                "1:uitchpn5MMGAQKBJh7bIr/bAr7s=",
            ),
            (
                (1, "1.2.3.4", "5.6.7.8", Some(8), Some(0), 1),
                "1:9pr4ZGTICiuZoIh90RRYE2RyXpU=",
            ),
            (
                (1, "5.6.7.8", "1.2.3.4", Some(0), Some(0), 1),
                "1:9pr4ZGTICiuZoIh90RRYE2RyXpU=",
            ),
            (
                (1, "1.2.3.4", "5.6.7.8", Some(11), Some(0), 1),
                "1:1DD7cWGC/Yg91YGsQeni8du3pIA=",
            ),
            (
                (1, "1.2.3.4", "5.6.7.8", None, None, 46),
                "1:/buhqeOmaRCopOZFy9HnoJd5XW8=",
            ),
            (
                (1, "5.6.7.8", "1.2.3.4", None, None, 46),
                "1:/buhqeOmaRCopOZFy9HnoJd5XW8=",
            ),
        ];
        raw.into_iter()
            .map(|(r, exp)| (r.into(), exp.to_string()))
            .collect()
    }

    #[test]
    fn test_baseline_seed_1() {
        for (input, exp) in test_baseline_ipv4_seed_data() {
            let v = calculate_ipv4_community_id(
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

    fn test_baseline_ipv4_disable_base64() -> Vec<(Ipv4Input, String)> {
        let raw = vec![
            (
                (0, "1.2.3.4", "5.6.7.8", Some(1122), Some(3344), 6),
                "1:c026f7386ef200559e95a53276ed03fb5db908b3",
            ),
            (
                (0, "5.6.7.8", "1.2.3.4", Some(3344), Some(1122), 6),
                "1:c026f7386ef200559e95a53276ed03fb5db908b3",
            ),
            (
                (0, "1.2.3.4", "5.6.7.8", Some(1122), Some(3344), 17),
                "1:d0cbbd227431eb3e1988264cffb1d78b658c84e8",
            ),
            (
                (0, "5.6.7.8", "1.2.3.4", Some(3344), Some(1122), 17),
                "1:d0cbbd227431eb3e1988264cffb1d78b658c84e8",
            ),
            (
                (0, "1.2.3.4", "5.6.7.8", Some(1122), Some(3344), 132),
                "1:10ab7832cc6ec9a13a98bfa19ab12443d72c0c3f",
            ),
            (
                (0, "5.6.7.8", "1.2.3.4", Some(3344), Some(1122), 132),
                "1:10ab7832cc6ec9a13a98bfa19ab12443d72c0c3f",
            ),
            (
                (0, "1.2.3.4", "5.6.7.8", Some(8), Some(0), 1),
                "1:72ba1d4472f6144b078dbbf752446b7dbb386d9d",
            ),
            (
                (0, "5.6.7.8", "1.2.3.4", Some(0), Some(0), 1),
                "1:72ba1d4472f6144b078dbbf752446b7dbb386d9d",
            ),
            (
                (0, "1.2.3.4", "5.6.7.8", Some(11), Some(0), 1),
                "1:7ff6224b25aa733ad381f5026650549ef1d170f9",
            ),
            (
                (0, "1.2.3.4", "5.6.7.8", None, None, 46),
                "1:8a4bf79267fcf65b9fef758fcf58ceb38f52efaf",
            ),
            (
                (0, "5.6.7.8", "1.2.3.4", None, None, 46),
                "1:8a4bf79267fcf65b9fef758fcf58ceb38f52efaf",
            ),
        ];
        raw.into_iter()
            .map(|(r, exp)| (r.into(), exp.to_string()))
            .collect()
    }

    #[test]
    fn test_baseline_disable_base64() {
        for (input, exp) in test_baseline_ipv4_disable_base64() {
            let v = calculate_ipv4_community_id(
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
