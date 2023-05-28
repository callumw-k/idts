use anyhow::Result;

use crate::{
    check_empty_args,
    date_helpers::get_date_values,
    file_helpers::{get_file_string, parse_file_string, write_file},
    Entry,
};

pub fn start(args: &[String]) -> Result<()> {
    check_empty_args(args);
    let (_, file_path, date_string) = get_date_values()?;
    let file_contents = get_file_string(&file_path)?;
    let mut data: Vec<Entry> =
        parse_file_string::<Entry>(&file_contents).unwrap_or_else(|_| Vec::new());

    if let Some(last_item) = data.last_mut() {
        if last_item.stop_date.is_none() {
            last_item.stop_date = Some(date_string.clone());
        }
    };
    data.push(Entry {
        id: (data.len() + 1) as u32,
        tags: args.to_vec(),
        start_date: date_string,
        stop_date: None,
    });
    write_file(&data, &file_path)?;
    Ok(())
}
