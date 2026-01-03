#![no_std]

cfg_if::cfg_if! {
    if #[cfg(feature = "ktf")] {
        pub mod ktf;
    } else if #[cfg(feature = "lgt")] {
        pub mod lgt;
    }
}

#[allow(dead_code)]
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
    pub fn handle_clet_event(r#type: i32, param1: i32, param2: i32);
}
