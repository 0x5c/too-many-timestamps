use std::fmt;

use chrono::{DateTime, Utc, SecondsFormat};
use crossterm::style::Stylize;


pub enum InputType {
    S,
    M,
    U,
    N,
    Discord,
    Twitter,
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Self::S => "seconds",
            Self::M => "millis",
            Self::U => "micros",
            Self::N => "nanos",
            Self::Discord => "Discord ID",
            Self::Twitter => "Twitter ID",
        };
        write!(f, "{}", text)
    }
}

pub enum TimeSource {
    Current,
    Input(InputType),
}

impl fmt::Display for TimeSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Self::Current => String::from("current time"),
            Self::Input(i) => format!("input (as {})", i),
        };
        write!(f, "{}", text)
    }
}

pub fn print_timestamp(time: DateTime<Utc>, source: TimeSource) {
    println!("{} {}", "Using".dark_green(), source.to_string().dark_green());

    print!("{}", "ISO 8601/RFC 3339: ".dark_cyan());
    println!("{}", time.to_rfc3339_opts(SecondsFormat::AutoSi, true).blue());

    print!("{}", "Unix timestamp: ".dark_cyan());
    println!("{}", time.timestamp().to_string().blue());

    print!("{}", "Unix ts. with millis: ".dark_cyan());
    println!("{}", time.timestamp_millis().to_string().blue());
}
