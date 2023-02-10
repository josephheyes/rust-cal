use ical;
use chrono::{DateTime, Local};
use std::io::BufReader;
use std::fs::File;

pub struct Calendar {
    pub events: Vec<Event>
}

pub struct Event {
    pub title: String,
    pub desc: String,
    pub t_start: DateTime<Local>,
    pub t_end: DateTime<Local>
}

fn main() {
    let buf = BufReader::new(File::open("/tmp/calendar.ics").unwrap());

    let reader = ical::IcalParser::new(buf);

    
}
