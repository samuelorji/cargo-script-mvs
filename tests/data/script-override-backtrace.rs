use std::env;

fn main() {
    println!("--output--");
    assert_eq!(env::var("RUST_BACKTRACE"), Ok("0".into()));
    panic!("a pink elephant!");
}

