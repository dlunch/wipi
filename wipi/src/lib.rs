#![cfg_attr(target_os = "none", no_std)]

#[cfg(all(feature = "ktf", feature = "lgt"))]
compile_error!("Cannot enable both 'ktf' and 'lgt' features at the same time");

#[cfg(target_os = "none")]
#[cfg(feature = "ktf")]
mod ktf;

#[cfg(target_os = "none")]
#[cfg(feature = "ktf")]
use self::ktf::wipic;

#[cfg(target_os = "none")]
#[cfg(feature = "lgt")]
mod lgt;

#[cfg(target_os = "none")]
#[cfg(feature = "lgt")]
use self::lgt::wipic;

#[cfg(not(target_os = "none"))]
mod emulation;

#[cfg(not(target_os = "none"))]
use self::emulation::wipic;

unsafe extern "C" {
    #[link_name = "startClet"]
    pub fn start_clet();
    #[link_name = "destroyClet"]
    pub fn destroy_clet();
    #[link_name = "paintClet"]
    pub fn paint_clet();
    #[link_name = "pauseClet"]
    pub fn pause_clet();
    #[link_name = "resumeClet"]
    pub fn resume_clet();
}

pub fn println(s: &str) {
    wipic::println(s)
}

pub fn exit(code: i32) {
    wipic::exit(code)
}
