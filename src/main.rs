use ical::{self, parser::ParserError};
use chrono::{DateTime, Local, Date};
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Default)]
pub struct Calendar {
    pub events: Vec<Event>
}

#[derive(Debug, Default)]
pub struct Event {
    pub title: String,
    pub desc: String,
    pub t_start: DateTime<Local>,
    // pub t_end: DateTime<Local>
}

fn main() {

    match parse() {
        Ok(data) => {
            for event in data.events {
                println!("{:?}", event);
            }
        }
        Err(error) => {
            println!("Error parsing file: {error}");
        }
    }
}

fn parse() -> Result<Calendar, ParserError>{
    let buf = BufReader::new(File::open("./calendar.ics").unwrap());
    let parser = ical::IcalParser::new(buf);
    let mut data = Calendar::default();

    for line in parser {
        match line {
            Ok(ical) => {
                for event in ical.events {
                    let mut constructed_event = Event::default();
                    let mut tz = String::new();
                    for property in event.properties {
                        if property.name == "SUMMARY" {
                            constructed_event.title = property.value.unwrap();
                        }
                        else if property.name == "DESCRIPTION" {
                            constructed_event.desc = property.value.unwrap();
                            
                        }
                        else if property.name == "DTSTART" {
                            let time_str = property.value.unwrap();

                            for (name, values) in property.params.unwrap() {
                                if name == "TZID" {
                                    tz = values[0].parse().unwrap();
                                }
                            }

                            match chrono::NaiveDateTime::parse_from_str(&time_str, "%Y%m%dT%H%M%S") {
                                Ok(time) => {
                                    
                                }
                                Err(error) => {

                                }
                            }
                        }
                    }
                    data.events.push(constructed_event);
                }
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
    
    return Ok(data);
}
