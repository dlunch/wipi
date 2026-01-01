mod lgt_elf;

use lgt_elf::fix_lgt_elf;

pub fn create_ktf_archive(
    executable_path: &str,
    main_class_name: &str,
    aid: &str,
    pid: &str,
    resource_path: Option<&str>,
) -> anyhow::Result<Vec<u8>> {
    use std::{
        fs,
        io::{Cursor, Write},
    };

    use zip::{write::SimpleFileOptions, ZipWriter};

    let executable_file = fs::read(executable_path)?;
    // we store bss size at last 4 bytes of the file
    let bss_size = u32::from_le_bytes(
        executable_file[executable_file.len() - 4..]
            .try_into()
            .expect("Invalid BSS size bytes"),
    );
    let jar = build_jar(
        &format!("client.bin{bss_size}"),
        executable_file,
        resource_path,
    )?;

    let mut archive = Vec::new();
    {
        let mut archive_zip = ZipWriter::new(Cursor::new(&mut archive));

        archive_zip.start_file("__adf__", SimpleFileOptions::default())?;
        archive_zip.write_all(build_metadata(aid, pid, main_class_name).as_bytes())?;
        archive_zip.start_file(format!("{aid}.jar"), SimpleFileOptions::default())?;
        archive_zip.write_all(&jar)?;
    }

    Ok(archive)
}

pub fn create_lgt_archive(
    executable_path: &str,
    main_class_name: &str,
    aid: &str,
    pid: &str,
    resource_path: Option<&str>,
) -> anyhow::Result<Vec<u8>> {
    use std::{
        fs,
        io::{Cursor, Write},
    };

    use zip::{write::SimpleFileOptions, ZipWriter};

    let executable_file = fs::read(executable_path)?;
    let fixed_executable = fix_lgt_elf(&executable_file)?;
    let jar = build_jar("binary.mod", fixed_executable, resource_path)?;

    let mut archive = Vec::new();
    {
        let mut archive_zip = ZipWriter::new(Cursor::new(&mut archive));

        archive_zip.start_file("app_info", SimpleFileOptions::default())?;
        archive_zip.write_all(build_metadata(aid, pid, main_class_name).as_bytes())?;
        archive_zip.start_file(format!("{aid}.jar"), SimpleFileOptions::default())?;
        archive_zip.write_all(&jar)?;
    }

    Ok(archive)
}

fn build_metadata(aid: &str, pid: &str, main_class_name: &str) -> String {
    format!(
        "\
            AID:{aid}\n\
            PID:{pid}\n\
            MClass:{main_class_name}\n\
        "
    )
}

fn build_jar(
    executable_name: &str,
    executable_file: Vec<u8>,
    resource_path: Option<&str>,
) -> anyhow::Result<Vec<u8>> {
    use std::{
        fs,
        io::{Cursor, Write},
    };

    use zip::{write::SimpleFileOptions, ZipWriter};

    let mut jar = Vec::new();
    {
        let mut jar_zip = ZipWriter::new(Cursor::new(&mut jar));
        jar_zip.start_file(executable_name, SimpleFileOptions::default())?;
        jar_zip.write_all(&executable_file)?;
        jar_zip.start_file("META-INF/MANIFEST.MF", SimpleFileOptions::default())?;
        jar_zip.write_all("Manifest-Version: 1.0\n".as_bytes())?;

        if let Some(resource_path) = resource_path {
            let resource_files = fs::read_dir(resource_path)?;
            for entry in resource_files {
                let path = entry?.path();

                if path.is_file() {
                    let file_name = path
                        .file_name()
                        .ok_or(anyhow::anyhow!("Invalid file name"))?
                        .to_str()
                        .ok_or(anyhow::anyhow!("Invalid UTF-8 in file name"))?;

                    jar_zip.start_file(file_name, SimpleFileOptions::default())?;
                    jar_zip.write_all(&fs::read(&path)?)?;
                }
            }
        }
    }

    Ok(jar)
}
