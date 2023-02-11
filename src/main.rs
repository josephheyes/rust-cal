use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use ical::{self};
use std::fmt::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Default)]
pub struct Calendar {
    pub events: Vec<Event>,
}

#[derive(Debug, Default)]
pub struct Event {
    pub title: String,
    pub desc: String,
    pub location: String,
    pub dt_start: NaiveDateTime,
    pub dt_end: NaiveDateTime,
}

fn main() {
    match parse() {
        Ok(data) => {
            for event in data.events {
                println!("{:#?}", event);
            }
        }
        Err(error) => {
            println!("Error parsing file: {error}");
        }
    }
}

fn format_datetime(time_string: String) -> NaiveDateTime {
    match chrono::NaiveDateTime::parse_from_str(&time_string, "%Y%m%dT%H%M%S") {
        Ok(datetime) => {
            return datetime;
        }
        Err(error) => {
            println!("Error reading DateTime: {error}\nUsing default DateTime");
            return NaiveDateTime::new(
                NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            );
        }
    }
}

fn parse() -> Result<Calendar, Error> {
    let buf = BufReader::new(File::open("calendars/timetable.ics").unwrap());
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
                            constructed_event.desc = property.value.unwrap();
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
