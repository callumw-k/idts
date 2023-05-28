use std::{fs::create_dir_all, path::PathBuf};

use anyhow::{anyhow, Context, Ok};
use chrono::{DateTime, Duration, Local};

pub fn get_duration(start_date: DateTime<Local>, end_date: DateTime<Local>) -> Duration {
    end_date - start_date
}

pub fn get_date_values() -> anyhow::Result<(DateTime<Local>, PathBuf, std::string::String)> {
    let date = Local::now();
    let file_name = date.format("%Y-%m-%d").to_string() + ".json";

    let home_dir = match std::env::var_os("HOME") {
        Some(dir) => dir,
        None => return Err(anyhow!("Can't find home directory")),
    };

    // Build the path to the file.
    let mut file_path = PathBuf::from(home_dir);
    file_path.push(".config");
    file_path.push("mytime");
    file_path.push("entries");

    if let Err(_) = create_dir_all(&file_path) {
        return Err(anyhow!("Something went wrong creating folder structure"));
    }

    file_path.push(&file_name);
    Ok((date, file_path, date.to_rfc3339()))
}

pub fn get_local_date(date: &str) -> anyhow::Result<DateTime<Local>> {
    Ok(DateTime::parse_from_rfc3339(date)
        .context("Error parsing date string from file")?
        .with_timezone(&Local))
}

pub fn get_pretty_duration(diff: Duration) -> String {
    let hours = diff.num_hours();
    let minutes = diff.num_minutes() % 60;
    let seconds = diff.num_seconds() % 60;
    format!("{}h {}m {}s", hours, minutes, seconds)
}
