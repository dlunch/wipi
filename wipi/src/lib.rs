#![cfg_attr(target_os = "none", no_std)]

cfg_if::cfg_if! {
    if #[cfg(all(feature = "ktf", feature = "lgt"))] {
        compile_error!("Cannot enable both 'ktf' and 'lgt' features at the same time");
    } else if #[cfg(all(not(feature = "ktf"), not(feature = "lgt"), target_os = "none"))] {
        compile_error!("At least one of 'ktf' or 'lgt' features must be enabled");
    }
}

cfg_if::cfg_if! {
    // common target modules
    if #[cfg(any(feature = "ktf", feature = "lgt"))] {
        mod panic_handler;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ktf")] {
        mod ktf;
        pub use self::ktf::wipic;
    } else if #[cfg(feature = "lgt")] {
        mod lgt;
        pub use self::lgt::wipic;
    } else if #[cfg(not(target_os = "none"))] {
        mod emulation;
        pub use self::emulation::wipic;
    }
}

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
    #[link_name = "handleCletEvent"]
    pub fn handle_clet_event();
}
