use std::{
    fs::{self},
    io::{Cursor, Write},
};

use zip::{ZipWriter, write::SimpleFileOptions};

pub fn create_ktf_archive(
    executable_path: &str,
    main_class_name: &str,
    aid: &str,
    pid: &str,
    resource_path: Option<&str>,
) -> anyhow::Result<Vec<u8>> {
    let mut jar = Vec::new();
    {
        let mut jar_zip = ZipWriter::new(Cursor::new(&mut jar));

        let executable_file = fs::read(executable_path)?;
        // we store bss size at last 4 bytes of the file
        let bss_size = u32::from_le_bytes(
            executable_file[executable_file.len() - 4..]
                .try_into()
                .expect("Invalid BSS size bytes"),
        );
        jar_zip.start_file(
            format!("client.bin{bss_size}"),
            SimpleFileOptions::default(),
        )?;
        jar_zip.write_all(&executable_file)?;
        jar_zip.start_file("META-INF/MANIFEST.MF", SimpleFileOptions::default())?;
        jar_zip.write_all("Manifest-Version: 1.0\n".as_bytes())?;
    }

    let mut archive = Vec::new();
    {
        let mut archive_zip = ZipWriter::new(Cursor::new(&mut archive));

        archive_zip.start_file("__adf__", SimpleFileOptions::default())?;
        archive_zip.write_all(build_adf(aid, pid, main_class_name).as_bytes())?;
        archive_zip.start_file(format!("{aid}.jar"), SimpleFileOptions::default())?;
        archive_zip.write_all(&jar)?;

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

                    archive_zip.start_file(file_name, SimpleFileOptions::default())?;
                    archive_zip.write_all(&fs::read(&path)?)?;
                }
            }
        }
    }

    Ok(archive)
}

fn build_adf(aid: &str, pid: &str, main_class_name: &str) -> String {
    format!(
        "\
            AID:{aid}\n\
            PID:{pid}\n\
            MClass:{main_class_name}\n\
        "
    )
}
