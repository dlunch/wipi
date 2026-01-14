#![cfg_attr(target_os = "none", no_std)]
#[cfg(not(target_os = "none"))]
pub fn add_wipi_build_args() {
    // add link arg for ktf.ld if the ktf feature is enabled
    #[cfg(feature = "ktf")]
    {
        let ktf_ld = include_bytes!("../../wipi_boot/src/ktf/ktf.ld");
        std::fs::write(std::env::var("OUT_DIR").unwrap() + "/ktf.ld", ktf_ld).unwrap();
        println!(
            "cargo:rustc-link-arg={}",
            std::env::var("OUT_DIR").unwrap() + "/ktf.ld"
        );
        println!(
            "cargo:rerun-if-changed={}",
            std::env::var("OUT_DIR").unwrap() + "/ktf.ld"
        );
    }
}
