fn main() {
    #[cfg(feature = "ktf")]
    println!("cargo::rerun-if-changed=src/ktf.ld");
}
