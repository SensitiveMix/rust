// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// FIXME(tschottdorf): this test should pass.

#![feature(match_default_bindings)]

#[derive(PartialEq, Eq)]
struct Foo {
    bar: i32,
}

const FOO: Foo = Foo{bar: 5};

fn main() {
    let f = Foo{bar:6};

    match &f {
        FOO => {}, //~ ERROR mismatched types
        _ => panic!(),
    }
}
