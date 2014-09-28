// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Simple backtrace functionality (to print on failure)

#![allow(non_camel_case_types)]

extern crate core;
use core::fmt::*;
use core::prelude::*;

use libc;
use monitor::*;



// All rust symbols are in theory lists of "::"-separated identifiers. Some
// assemblers, however, can't handle these characters in symbol names. To get
// around this, we use C++-style mangling. The mangling method is:
//
// 1. Prefix the symbol with "_ZN"
// 2. For each element of the path, emit the length plus the element
// 3. End the path with "E"
//
// For example, "_ZN4testE" => "test" and "_ZN3foo3bar" => "foo::bar".
//
// We're the ones printing our backtraces, so we can't rely on anything else to
// demangle our symbols. It's *much* nicer to look at demangled symbols, so
// this function is implemented to give us nice pretty output.
//
// Note that this demangler isn't quite as fancy as it could be. We have lots
// of other information in our symbols like hashes, version, type information,
// etc. Additionally, this doesn't handle glue symbols at all.
pub fn demangle(writer: &mut B512Writer, s: &str) -> Result<(),FormatError> {
    // First validate the symbol. If it doesn't look like anything we're
    // expecting, we just print it literally. Note that we must handle non-rust
    // symbols because we could have any function in the backtrace.
    let mut valid = true;
    if s.len() > 4 && s.starts_with("_ZN") {
        let mut chars = s.slice(3, s.len() - 1).chars();
        while valid {
            let mut i = 0;
            for c in chars {
                if c.is_digit() {
                    i = i * 10 + c as uint - '0' as uint;
                } else {
                    break
                }
            }
            if i == 0 {
                valid = chars.next().is_none();
                break
            } else if chars.by_ref().take(i - 1).count() != i - 1 {
                valid = false;
            }
        }
    } else {
        valid = false;
    }

    // Alright, let's do this.
    if !valid {
        try!(writer.write_str(s));
    } else {
        let mut s = s.slice_from(3);
        let mut first = true;
        while s.len() > 1 {
            if !first {
                try!(writer.write_str("::"));
            } else {
                first = false;
            }
            let mut rest = s;
            let mut num = 0;
            while rest.char_at(0).is_digit() {
                num = num*10 + rest.char_at(0) as u8 - '0' as u8;
                rest = rest.slice_from(1);
            }
            let i: uint = num as uint; //from_str(s.slice_to(s.len() - rest.len())).unwrap();
            s = rest.slice_from(i);
            rest = rest.slice_to(i);
            while rest.len() > 0 {
                if rest.starts_with("$") {
                    macro_rules! demangle(
                        ($($pat:expr => $demangled:expr),*) => ({
                            $(if rest.starts_with($pat) {
                                try!(writer.write_str($demangled));
                                rest = rest.slice_from($pat.len());
                              } else)*
                            {
                                try!(writer.write_str(rest));
                                break;
                            }

                        })
                    )
                    // see src/librustc/back/link.rs for these mappings
                    demangle! (
                        "$SP$" => "@",
                        "$UP$" => "Box",
                        "$RP$" => "*",
                        "$BP$" => "&",
                        "$LT$" => "<",
                        "$GT$" => ">",
                        "$LP$" => "(",
                        "$RP$" => ")",
                        "$C$"  => ",",

                        // in theory we can demangle any Unicode code point, but
                        // for simplicity we just catch the common ones.
                        "$x20" => " ",
                        "$x27" => "'",
                        "$x5b" => "[",
                        "$x5d" => "]"
                    )
                } else {
                    let idx = match rest.find('$') {
                        None => rest.len(),
                        Some(i) => i,
                    };
                    try!(writer.write_str(rest.slice_to(idx)));
                    rest = rest.slice_from(idx);
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use prelude::*;
    use B512Writer;

    macro_rules! t( ($a:expr, $b:expr) => ({
        let mut m = B512Writer::new();
        super::demangle(&mut m, $a).unwrap();
        assert_eq!(String::from_utf8(m.unwrap()).unwrap(), $b.to_string());
    }) )

    #[test]
    fn demangle() {
        t!("test", "test");
        t!("_ZN4testE", "test");
        t!("_ZN4test", "_ZN4test");
        t!("_ZN4test1a2bcE", "test::a::bc");
    }

    #[test]
    fn demangle_dollars() {
        t!("_ZN4$UP$E", "Box");
        t!("_ZN8$UP$testE", "Boxtest");
        t!("_ZN8$UP$test4foobE", "Boxtest::foob");
        t!("_ZN8$x20test4foobE", " test::foob");
    }

    #[test]
    fn demangle_many_dollars() {
        t!("_ZN12test$x20test4foobE", "test test::foob");
        t!("_ZN12test$UP$test4foobE", "testBoxtest::foob");
    }
}
