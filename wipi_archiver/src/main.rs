#![cfg_attr(target_os = "none", no_std)]

#[cfg(target_os = "none")]
pub fn main() {}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(not(target_os = "none"))]
pub fn main() -> anyhow::Result<()> {
    use std::io::{Write, stdout};

    use clap::Parser;

    use wipi_archiver::{create_ktf_archive, create_lgt_archive};

    #[derive(Parser)]
    struct Args {
        target: String,
        executable_path: String,
        main_class_name: String,
        aid: String,
        pid: String,
        resource_path: Option<String>,
    }

    let args = Args::parse();

    let archive = if args.target == "ktf" {
        create_ktf_archive(
            &args.executable_path,
            &args.main_class_name,
            &args.aid,
            &args.pid,
            args.resource_path.as_deref(),
        )?
    } else if args.target == "lgt" {
        create_lgt_archive(
            &args.executable_path,
            &args.main_class_name,
            &args.aid,
            &args.pid,
            args.resource_path.as_deref(),
        )?
    } else {
        return Err(anyhow::anyhow!("Unsupported target: {}", args.target));
    };

    stdout().write_all(&archive)?;

    Ok(())
}
