use anyhow::{anyhow, Ok};
use chrono::Local;
use date_helpers::{get_date_values, get_duration, get_local_date, get_pretty_duration};
use file_helpers::{get_file_string, parse_file_string};
#[macro_use]
extern crate prettytable;
use prettytable::Table;
use serde::{Deserialize, Serialize};
use start::start;
use std::{env, eprintln, format, println};
use stop::stop;

mod date_helpers;
mod file_helpers;
mod start;
mod stop;

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry {
    id: u32,
    tags: Vec<String>,
    start_date: String,
    stop_date: Option<String>,
}

fn check_empty_args(args: &[String]) {
    if args.is_empty() {
        eprintln!("Please add at least one task name.");
        std::process::exit(1)
    }
}

fn summary() -> anyhow::Result<()> {
    let (_, file_name, _) = get_date_values()?;

    let file_contents = get_file_string(&file_name)?;

    if file_contents.is_empty() {
        return Err(anyhow!(
            "Please create at least one time entry before stopping"
        ));
    }
    let data = parse_file_string::<Entry>(&file_contents)?;

    let mut table = Table::new();
    table.add_row(row!["Tag", "ID", "Start time", "Stop time", "Duration"]);
    for entry in &data {
        let start_date = get_local_date(&entry.start_date)?;
        let tag_string = format!("{:?}", entry.tags);
        if let Some(stop_date) = &entry.stop_date {
            let stop_date = get_local_date(stop_date)?;
            let diff = get_duration(start_date, stop_date);
            table.add_row(row![
                tag_string,
                entry.id,
                start_date,
                stop_date,
                get_pretty_duration(diff)
            ]);
        } else {
            table.add_row(row![
                tag_string,
                entry.id,
                start_date,
                "",
                get_pretty_duration(get_duration(start_date, Local::now()))
            ]);
        }
    }
    table.printstd();
    Ok(())
}

fn main() {
    let mut args = Vec::new();
    for arg in env::args().skip(1) {
        args.push(arg);
    }
    if args.len() < 1 {
        println!("Please provide a CLI option");
        return;
    }

    let init_arg = args.get(0).unwrap();

    match init_arg.as_str() {
        "start" => {
            if let Err(err) = start(&args[1..]) {
                eprintln!("An error occured: {:#?}", err)
            }
        }
        "stop" => {
            if let Err(err) = stop() {
                eprintln!("{:#?}", err)
            }
        }
        "summary" => {
            if let Err(err) = summary() {
                eprintln!("{:#?}", err)
            }
        }
        _ => println!("Didn't work"),
    }
}
