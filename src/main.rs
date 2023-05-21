use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};
use std::{env, fs::OpenOptions, io::Read, println};

#[derive(Deserialize, Serialize, Debug)]
struct Entry {
    id: u32,
    tags: Vec<String>,
    start_date: String,
    stop_date: Option<String>,
}

fn get_file_string(file_name: &str) -> std::string::String {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_name)
        .expect("Failed to open or create file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Failed to read file");
    file_contents
}

fn write_file<T: Serialize>(data: &T, file_name: &str) {
    std::fs::write(
        file_name,
        serde_json::to_string(data).expect("Something went wrong parsing to JSON"),
    )
    .expect("Something went wrong writing to file")
}

fn get_duration(start_date: DateTime<Local>, end_date: DateTime<Local>) -> Duration {
    end_date - start_date
}

fn get_date_values() -> (DateTime<Local>, std::string::String, std::string::String) {
    let date = Local::now();
    let file_name = date.format("%Y-%m-%d").to_string() + ".json";
    (date, file_name, date.to_rfc3339())
}

fn check_empty_args(args: &[String]) {
    if args.is_empty() {
        eprintln!("Please add at least one task name.");
        std::process::exit(1)
    }
}

fn get_local_date(date: &str) -> DateTime<Local> {
    DateTime::parse_from_rfc3339(date)
        .expect("Error parsing date string from file")
        .with_timezone(&Local)
}

fn start(args: &[String]) {
    check_empty_args(args);
    let (_, file_name, date_string) = get_date_values();
    let file_contents = get_file_string(&file_name);
    if file_contents.is_empty() {
        let entry = Entry {
            id: 1,
            tags: args.to_vec(),
            start_date: date_string,
            stop_date: None,
        };
        write_file(&vec![entry], &file_name)
    } else {
        let mut data = serde_json::from_str::<Vec<Entry>>(&file_contents)
            .expect("Something went wrong parsing JSON from file");
        data.push(Entry {
            id: (data.len() + 1) as u32,
            tags: args.to_vec(),
            start_date: date_string,
            stop_date: None,
        });
        write_file(&data, &file_name)
    }
}

fn stop() {
    let (date, file_name, date_string) = get_date_values();
    let file_contents = get_file_string(&file_name);

    if file_contents.is_empty() {
        eprint!("Please create at least one time entry before stoping");
        std::process::exit(1)
    }

    let mut data = serde_json::from_str::<Vec<Entry>>(&file_contents)
        .expect("Something went wrong parsing JSON");

    let last_entry = data
        .pop()
        .expect("Please create at least one time entry before stopping");

    if last_entry.stop_date.is_some() {
        eprintln!("Nothing to stop");
        std::process::exit(1);
    }

    let start_date = last_entry.start_date.clone();

    let entry = Entry {
        id: last_entry.id,
        tags: last_entry.tags,
        start_date,
        stop_date: Some(date_string),
    };
    data.push(entry);
    let start_date = get_local_date(&last_entry.start_date);
    let diff = get_duration(start_date, date);
    write_file(&data, &file_name);
}

fn summary() {
    let (_, file_name, _) = get_date_values();
    let file_contents = get_file_string(&file_name);
}

fn main() {
    let mut args = Vec::new();
    for arg in env::args().skip(1) {
        args.push(arg);
    }

    let init_arg = args.get(0).expect("Please provide a cli option");

    match init_arg.as_str() {
        "start" => start(&args[1..]),
        "stop" => stop(),
        _ => println!("Didn't work"),
    }
}
