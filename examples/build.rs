fn main() {
    #[cfg(feature = "ktf")]
    println!("cargo::rustc-link-arg=wipi_boot/src/ktf/ktf.ld");
}
