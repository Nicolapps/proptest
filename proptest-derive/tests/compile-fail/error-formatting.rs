// Copyright 2025 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate proptest_derive;
use proptest_derive::Arbitrary;

fn main() {}

#[derive(Arbitrary)] //~ ERROR: [proptest_derive, E0002] during #[derive(Arbitrary)]: Deriving is only possible for structs and enums. It is currently not defined unions. Please see: https://proptest-rs.github.io/proptest/proptest-derive/errors.html#e0002 for more information.
union Foo {
    x: usize,
}
