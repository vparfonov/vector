// Copyright 2019 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(warnings)]

mod util;

use std::{marker::PhantomData, option::IntoIter};

use {static_assertions::assert_impl_all, zerocopy::FromBytes};

use crate::util::AU16;

// A struct is `FromBytes` if:
// - all fields are `FromBytes`

#[derive(FromBytes)]
struct Zst;

assert_impl_all!(Zst: FromBytes);

#[derive(FromBytes)]
struct One {
    a: u8,
}

assert_impl_all!(One: FromBytes);

#[derive(FromBytes)]
struct Two {
    a: u8,
    b: Zst,
}

assert_impl_all!(Two: FromBytes);

#[derive(FromBytes)]
struct Unsized {
    a: [u8],
}

assert_impl_all!(Unsized: FromBytes);

#[derive(FromBytes)]
struct TypeParams<'a, T: ?Sized, I: Iterator> {
    a: I::Item,
    b: u8,
    c: PhantomData<&'a [u8]>,
    d: PhantomData<&'static str>,
    e: PhantomData<String>,
    f: T,
}

assert_impl_all!(TypeParams<'static, (), IntoIter<()>>: FromBytes);
assert_impl_all!(TypeParams<'static, AU16, IntoIter<()>>: FromBytes);
assert_impl_all!(TypeParams<'static, [AU16], IntoIter<()>>: FromBytes);

// Deriving `FromBytes` should work if the struct has bounded parameters.

#[derive(FromZeroes, FromBytes)]
#[repr(transparent)]
struct WithParams<'a: 'b, 'b: 'a, const N: usize, T: 'a + 'b + FromBytes>(
    [T; N],
    PhantomData<&'a &'b ()>,
)
where
    'a: 'b,
    'b: 'a,
    T: 'a + 'b + FromBytes;

assert_impl_all!(WithParams<'static, 'static, 42, u8>: FromBytes);
