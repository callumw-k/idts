use anyhow::{anyhow, Context, Ok};

use crate::{
    date_helpers::{get_date_values, get_duration, get_local_date, get_pretty_duration},
    file_helpers::{get_file_string, parse_file_string, write_file},
    Entry,
};

pub fn stop() -> anyhow::Result<()> {
    let (date, file_name, date_string) = get_date_values()?;

    let file_contents = get_file_string(&file_name)?;
    if (&file_contents).is_empty() {
        return Err(anyhow!(
            "Please create at least one time entry before stopping"
        ));
    }
    let mut data = parse_file_string::<Entry>(&file_contents)?;

    let last_entry = data
        .pop()
        .context("Can't retrieve the last item of the array")?;

    if last_entry.stop_date.is_some() {
        return Err(anyhow!("Nothing to stop"));
    }

    let start_date = &last_entry.start_date;

    data.push(Entry {
        id: last_entry.id,
        tags: last_entry.tags.clone(),
        start_date: start_date.to_string(),
        stop_date: Some(date_string),
    });

    let start_date = get_local_date(&last_entry.start_date)?;

    let diff = get_duration(start_date, date);

    println!(
        "You worked on {:?} for {}",
        &last_entry.tags,
        get_pretty_duration(diff)
    );

    write_file(&data, &file_name)?;
    Ok(())
}
