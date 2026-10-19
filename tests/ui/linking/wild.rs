//@ revisions: enabled override_lld
//@ run-pass
//@ needs-wild
//@ only-x86_64-unknown-linux-gnu
//@ compile-flags: -Zunstable-options
//@[enabled] compile-flags: -Clinker-features=+wild
//@[override_lld] compile-flags: -Clinker-features=+lld,-lld,+wild

fn main() {
    assert_eq!(format!("{}", 42), "42");
}
