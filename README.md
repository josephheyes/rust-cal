# rust-cal
A quick project I built to display my university timetable in the terminal using Rust

## How it works

Currently the program uses the reqwest crate to make a GET request to the university's iCalendar server, downloads the .ics file and uses the ical crate to parse it
