use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use ical::{self};
use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufReader};
use std::{env, vec};
use tabled::object::Rows;
use tabled::{Modify, Table, Tabled, Width};

#[derive(Default)]
pub struct Calendar {
    pub events: Vec<Event>,
}

#[derive(Debug, Default, Tabled)]
pub struct Event {
    #[tabled(rename = "Module")]
    pub title: String,
    #[tabled(rename = "Convener")]
    pub desc: String,
    #[tabled(rename = "Location")]
    pub location: String,
    #[tabled(skip)]
    pub dt_start: NaiveDateTime,
    #[tabled(skip)]
    pub dt_end: NaiveDateTime,
    #[tabled(rename = "Duration")]
    pub duration: String,
}

fn help() {
    println!("usage:
        rust-cal help
            Show this menu
        rust-cal today
            Show today's events. 'today' is optional
        rust-cal tomorrow
            Show tomorrow's events."
    )
}

fn main() {
    download_calendar();
    let args: Vec<String> = env::args().collect();
    let cal = match parse() {
        Ok(data) => data,
        Err(error) => {
            println!("Error parsing file: {error}");
            std::process::exit(-1);
        }
    };

    // Awful code
    let mut timetable: Vec<Event> = vec![];
    let mut display_table = false;
    match args.len() {
        // no args (get events for today's date)
        1 => {
            timetable = get_events(cal, Local::now().date_naive());
            display_table = true;
        }
        // 1 arg
        2 => {
            match args[1].as_str() {
                "today" => {
                    timetable = get_events(cal, Local::now().date_naive());
                    display_table = true;
                },
                "tomorrow" => {
                    let today = Local::now();
                    let tomorrow: NaiveDate = (today + Duration::days(1)).date_naive();
                    timetable = get_events(cal, tomorrow);
                    display_table = true;
                }
                "help" => {
                    help();
                }
                // Unsupported args
                _ => {
                    println!("Not a valid argument");
                    help();
                }
            }
        }
        // Unsupported number of args
        _ => {
            println!("Too many arguments");
            help();
        }
    };

    if display_table {
        let table = Table::new(timetable)
            .with(Modify::new(Rows::new(1..)).with(Width::wrap(20).keep_words()))
            .to_string();
        println!("{table}");
    }
}

fn get_events(cal: Calendar, date: NaiveDate) -> Vec<Event> {
    let mut events: Vec<Event> = vec![];
    // let date = Local::now().date_naive();

    for event in cal.events {
        if event.dt_start.date() == date {
            events.push(event);
        }
    }
    return events;
}

// Takes a string of format "YYYYMMDDTHHMMSS" and returns a NaiveDateTime
// (because using timezone is out of scope for my current uses)
// TODO: Obviously don't return default date if error reading it (as rare as this may be) as the event would not be displayed (when timetable printing functionality is added)
fn format_datetime(time_string: String) -> NaiveDateTime {
    match chrono::NaiveDateTime::parse_from_str(&time_string, "%Y%m%dT%H%M%S") {
        Ok(datetime) => return datetime,
        Err(error) => {
            println!("Error reading DateTime: {error}\nUsing default DateTime");
            return NaiveDateTime::new(
                NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            );
        }
    }
}

// Makes GET request to the UoN iCalendar server to download the timetable
fn download_calendar() {
    let res = reqwest::blocking::get(
        "https://ical.mycal.nottingham.ac.uk/e597c0a6-23c3-11ed-9b98-0050569f01bd",
    )
    .expect("request failed")
    .text()
    .expect("invalid body");
    let mut cal = File::create("timetable.ics").expect("file creation failure");
    io::copy(&mut res.as_bytes(), &mut cal).expect("Failed to copy body to file");
}

// Main parsing function
// Uses the IcalParser and extracts only the necessary values to display and sets them in custom constructed Event structs
// TODO: Better error handling
fn parse() -> Result<Calendar, Error> {
    let buf = BufReader::new(File::open("timetable.ics").unwrap());
    let parser = ical::IcalParser::new(buf);
    let mut data = Calendar::default();

    for line in parser {
        match line {
            Ok(ical) => {
                for event in ical.events {
                    let mut constructed_event = Event::default();
                    // let mut tz: String;
                    for property in event.properties {
                        if property.name == "SUMMARY" && property.value.is_some() {
                            constructed_event.title = property.value.unwrap();
                        } else if property.name == "DESCRIPTION" && property.value.is_some() {
                            let desc = property.value.unwrap();
                            constructed_event.desc = desc
                                [desc.find(":").unwrap() + 2..desc.find("\\n").unwrap()]
                                .to_string();
                        } else if property.name == "LOCATION" && property.value.is_some() {
                            constructed_event.location = property.value.unwrap();
                        } else if property.name == "DTSTART" && property.value.is_some() {
                            let time_str = property.value.unwrap();
                            constructed_event.dt_start = format_datetime(time_str);
                        } else if property.name == "DTEND" && property.value.is_some() {
                            let time_str = property.value.unwrap();
                            constructed_event.dt_end = format_datetime(time_str);
                        }
                    }
                    constructed_event.duration = format!(
                        "{}-{}",
                        constructed_event.dt_start.time().to_string(),
                        constructed_event.dt_end.time().to_string()
                    );
                    data.events.push(constructed_event);
                }
            }
            Err(_) => {
                return Err(Error);
            }
        }
    }

    return Ok(data);
}
