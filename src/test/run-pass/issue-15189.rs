// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-pretty

macro_rules! third {
    ($e:expr) => ({let x = 2; $e[x]})
}

fn main() {
    let x = vec!(10_usize,11_usize,12_usize,13_usize);
    let t = third!(x);
    assert_eq!(t,12_usize);
}
