// Copyright 2020 TiKV Project Authors. Licensed under MIT or Apache-2.0.

//! x86 (32-bit) implementation of the PCLMULQDQ-based CRC calculation.

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::ops::BitXor;

#[cfg(feature = "vpclmulqdq")]
pub mod vpclmulqdq;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Simd(__m128i);

impl super::SimdExt for Simd {
    fn is_supported() -> bool {
        is_x86_feature_detected!("pclmulqdq") // _mm_clmulepi64_si128
            && is_x86_feature_detected!("sse2") // (all other _mm_*)
            && is_x86_feature_detected!("sse4.1")
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn new(high: u64, low: u64) -> Self {
        // On 32-bit systems, we need to split u64 into low and high 32-bit parts
        let high_low = (high & 0xFFFFFFFF) as i32;
        let high_high = ((high >> 32) & 0xFFFFFFFF) as i32;
        let low_low = (low & 0xFFFFFFFF) as i32;
        let low_high = ((low >> 32) & 0xFFFFFFFF) as i32;

        // Create the 128-bit vector using 32-bit parts
        Self(_mm_set_epi32(high_high, high_low, low_high, low_low))
    }

    #[inline]
    #[target_feature(enable = "sse2", enable = "pclmulqdq")]
    unsafe fn fold_16(self, coeff: Self) -> Self {
        let h = Self(_mm_clmulepi64_si128::<0x11>(self.0, coeff.0));
        let l = Self(_mm_clmulepi64_si128::<0x00>(self.0, coeff.0));
        h ^ l
    }

    #[inline]
    #[target_feature(enable = "sse2", enable = "pclmulqdq")]
    unsafe fn fold_8(self, coeff: u64) -> Self {
        let coeff = Self::new(0, coeff);
        let h = Self(_mm_clmulepi64_si128::<0x00>(self.0, coeff.0));
        let l = Self(_mm_srli_si128::<8>(self.0));
        h ^ l
    }

    #[inline]
    #[target_feature(enable = "sse2", enable = "sse4.1", enable = "pclmulqdq")]
    unsafe fn barrett(self, poly: u64, mu: u64) -> u64 {
        let polymu = Self::new(poly, mu);
        let t1 = _mm_clmulepi64_si128::<0x00>(self.0, polymu.0);
        let h = Self(_mm_slli_si128::<8>(t1));
        let l = Self(_mm_clmulepi64_si128::<0x10>(t1, polymu.0));
        let reduced = h ^ l ^ self;
        _mm_extract_epi64::<1>(reduced.0) as u64
    }
}

impl BitXor for Simd {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Self(unsafe { _mm_xor_si128(self.0, other.0) })
    }
}
