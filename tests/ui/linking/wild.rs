//@ run-pass
//@ needs-wild
//@ only-x86_64-unknown-linux-gnu
//@ compile-flags: -Zunstable-options -Clinker-features=+wild

fn main() {
    assert_eq!(format!("{}", 42), "42");
}
