use ical::{self, parser::ParserError};
use chrono::{DateTime, Local};
use std::io::BufReader;
use std::fs::File;

#[derive(Default)]
pub struct Calendar {
    pub events: Vec<Event>
}

#[derive(Default)]
pub struct Event {
    pub title: String,
    pub desc: String,
    // pub t_start: DateTime<Local>,
    // pub t_end: DateTime<Local>
}

fn main() {

    match parse() {
        Ok(data) => {
            for event in data.events {
                println!("{:?}", event.title);
            }
        }
        Err(error) => {
            return;
        }
    }
}

fn parse() -> Result<Calendar, ParserError>{
    let buf = BufReader::new(File::open("/tmp/calendar.ics").unwrap());
    let parser = ical::IcalParser::new(buf);
    let mut data = Calendar::default();

    for line in parser {
        match line {
            Ok(ical) => {
                for event in ical.events {
                    let mut constructed_event = Event::default();
                    for property in event.properties {
                        if property.name == "SUMMARY" {
                            constructed_event.title = property.value.unwrap();
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
