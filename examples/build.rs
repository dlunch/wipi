fn main() {
    #[cfg(feature = "ktf")]
    println!("cargo::rustc-link-arg=wipi/src/ktf.ld");
}
