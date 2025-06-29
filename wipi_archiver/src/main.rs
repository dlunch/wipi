use std::io::{Write, stdout};

use clap::Parser;

use wipi_archiver::create_ktf_archive;

#[derive(Parser)]
struct Args {
    executable_path: String,
    main_class_name: String,
    aid: String,
    pid: String,
    resource_path: Option<String>,
}

pub fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let archive = create_ktf_archive(
        &args.executable_path,
        &args.main_class_name,
        &args.aid,
        &args.pid,
        args.resource_path.as_deref(),
    )?;

    stdout().write_all(&archive)?;

    Ok(())
}
