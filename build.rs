use std::env;

fn main() {
    println!("cargo:rustc-env=PROFILE={}", env::var("PROFILE").unwrap());
}

