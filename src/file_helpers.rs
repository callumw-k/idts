use std::{fs::OpenOptions, io::Read, path::PathBuf};

use anyhow::{Context, Ok};
use serde::{de::DeserializeOwned, Serialize};

pub fn get_file_string(file_path: &PathBuf) -> anyhow::Result<String> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .context("Error reading/opening/creating file")?;

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .context("Error reading file to string")?;

    Ok(file_contents)
}

pub fn write_file<T: Serialize>(data: &T, file_path: &PathBuf) -> anyhow::Result<()> {
    std::fs::write(
        file_path,
        serde_json::to_string(data).context("Something went wrong parsing to string")?,
    )
    .context("Something went wrong writing to file")?;
    Ok(())
}

pub fn parse_file_string<T: DeserializeOwned>(file_contents: &String) -> anyhow::Result<Vec<T>> {
    serde_json::from_str::<Vec<T>>(&file_contents).context("Something went wrong parsing JSON")
}
