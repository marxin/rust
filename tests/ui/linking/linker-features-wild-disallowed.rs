//@ revisions: conflicting unsupported unstable_positive unstable_negative
//@ check-fail
//@ needs-llvm-components: x86
//@ dont-require-annotations: ERROR
//@ compile-flags: --crate-type=rlib
//@[conflicting] compile-flags: -Zunstable-options -Clinker-features=+lld,+wild
//@[unsupported] compile-flags: -Zunstable-options --target=i686-unknown-linux-gnu -Clinker-features=+wild
//@[unstable_positive] compile-flags: --target=x86_64-unknown-linux-gnu -Clinker-features=+wild
//@[unstable_negative] compile-flags: --target=x86_64-unknown-linux-gnu -Clinker-features=-wild

#![feature(no_core)]
#![no_core]
