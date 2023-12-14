use std::{
    fs::{self, File},
    io::{self, Cursor},
    path::Path,
};

use bytes::Bytes;
use zip::ZipArchive;

use super::get_client;

pub async fn get_zip(url: &str) -> anyhow::Result<ZipArchive<Cursor<Bytes>>> {
    let bytes = get_client()
        .get(url)
        .header(reqwest::header::ACCEPT, "application/zip")
        .send()
        .await?
        .bytes()
        .await?;

    Ok(ZipArchive::new(Cursor::new(bytes))?)
}

pub fn write_archive_to_path(
    path: &Path,
    mut archive: ZipArchive<Cursor<Bytes>>,
) -> anyhow::Result<()> {
    for index in 0..archive.len() {
        let mut zip_file = archive.by_index(index)?;

        let file_path = path.join(
            zip_file
                .enclosed_name()
                .ok_or(anyhow::anyhow!("Could not get file name"))?,
        );

        if zip_file.is_dir() {
            fs::create_dir_all(file_path)?;
            continue;
        }

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut fs_file = File::create(file_path)?;

        io::copy(&mut zip_file, &mut fs_file)?;
    }

    Ok(())
}

pub fn extract_libs(
    archive: &mut ZipArchive<Cursor<Bytes>>,
) -> anyhow::Result<Vec<(String, usize)>> {
    let mut res = Vec::new();

    for zip_index in 0..archive.len() {
        let file = archive.by_index(zip_index)?;
        let file_name = file
            .enclosed_name()
            .ok_or(anyhow::anyhow!("Could not get file name"))?
            .file_name()
            .ok_or(anyhow::anyhow!("Could not get last part of file name"))?
            .to_str()
            .ok_or(anyhow::anyhow!(
                "Could not convert os string to string in file name"
            ))?;

        if !file_name.starts_with("lib") || file_name.contains("debug") {
            continue;
        }
        let file_name = file_name.trim_start_matches("lib");

        let mut index = file_name.len() - 1;
        let chars = file_name.chars().collect::<Vec<_>>();
        for (idx, char) in chars.iter().enumerate().rev() {
            if *char == 'o' && chars[idx - 1] == 's' {
                index = idx;
                break;
            }
        }

        let file_name = &file_name[..index + 1];

        if !file_name.ends_with(".so") {
            continue;
        }

        res.push((file_name.trim_end_matches(".so").to_string(), zip_index));
    }

    Ok(res)
}
