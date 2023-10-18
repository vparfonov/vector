#![allow(non_upper_case_globals)]

use std::collections::HashMap;

use lazy_static::lazy_static;
use num_enum::{IntoPrimitive, TryFromPrimitive};

// https://github.com/corelight/pycommunityid/blob/master/communityid/icmp6.py
lazy_static! {
    static ref ICMP_TYPE_MAPPING: HashMap<IcmpType, IcmpType> = HashMap::from([
        (IcmpType::EchoRequest, IcmpType::EchoReply),
        (IcmpType::EchoReply, IcmpType::EchoRequest),
        (IcmpType::MldListenerQuery, IcmpType::MldListenerReport),
        (IcmpType::MldListenerReport, IcmpType::MldListenerQuery),
        (IcmpType::NdRouterSolicit, IcmpType::NdRouterAdvert),
        (IcmpType::NdRouterAdvert, IcmpType::NdRouterSolicit),
        (IcmpType::NdNeighborSolicit, IcmpType::NdNeighborAdvert),
        (IcmpType::NdNeighborAdvert, IcmpType::NdNeighborSolicit),
        (IcmpType::WruRequest, IcmpType::WruReply),
        (IcmpType::WruReply, IcmpType::WruRequest),
        (IcmpType::HaadRequest, IcmpType::HaadReply),
        (IcmpType::HaadReply, IcmpType::HaadRequest),
    ]);
}

// https://github.com/corelight/pycommunityid/blob/master/communityid/icmp6.py
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
enum IcmpType {
    EchoRequest = 128,
    EchoReply = 129,
    MldListenerQuery = 130,
    MldListenerReport = 131,
    NdRouterSolicit = 133,
    NdRouterAdvert = 134,
    NdNeighborSolicit = 135,
    NdNeighborAdvert = 136,
    WruRequest = 139,
    WruReply = 140,
    HaadRequest = 144,
    HaadReply = 145,
}

pub(crate) fn get_port_equivalents(mtype: u16, mcode: u16) -> (u16, u16, bool) {
    match IcmpType::try_from(mtype) {
        Ok(mtype_obj) => match ICMP_TYPE_MAPPING.get(&mtype_obj) {
            Some(v) => return (mtype, (*v).into(), false),
            None => return (mtype, mcode, true),
        },
        Err(_) => return (mtype, mcode, true),
    }
}
