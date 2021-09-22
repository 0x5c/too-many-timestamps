/*
    Too Many Timestamps
    Copyright (c) 2021 0x5c
    SPDX-License-Identifier: LiLiQ-Rplus-1.1
*/

use std::fmt;

use chrono::{
    DateTime,
    Utc,
};

pub type DT = DateTime<Utc>;

#[derive(Debug, PartialEq)]
pub enum InputType {
    Seconds,
    Millis,
    Micros,
    Nanos,
    Discord,
    Twitter,
}

impl InputType {
    pub fn from_letter(letter: &str) -> Self {
        match letter {
            "s" => InputType::Seconds,
            "m" => InputType::Millis,
            "u" => InputType::Micros,
            "n" => InputType::Nanos,
            "d" => InputType::Discord,
            "t" => InputType::Twitter,
            &_  => panic!("invalid letter for InputType"),
        }
    }

    pub fn default() -> Self {
        Self::Seconds
    }
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Self::Seconds => "seconds",
            Self::Millis => "millis",
            Self::Micros => "micros",
            Self::Nanos => "nanos",
            Self::Discord => "Discord ID",
            Self::Twitter => "Twitter ID",
        };
        write!(f, "{}", text)
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Operation {
    Timestamp(DT),
    Discord(DiscordID),
    Twitter(TwitterSnowflake),
}

#[derive(Debug)]
pub struct DiscordID {
    /// ID creation time
    pub time: DT,
    /// Internal Worker ID
    pub wid: u8,
    /// Internal Process ID
    pub pid: u8,
    /// Sequential
    pub seq: u16,
}

#[derive(Debug)]
pub struct TwitterSnowflake {
    /// Snowflake creation time
    pub time: DT,
    /// Internal machine ID
    pub mid: u16,
    /// Sequential
    pub seq: u16,
}


#[cfg(test)]
mod tests {
    use super::InputType;

    #[test]
    fn inputtype_from_letter() {
        assert_eq!(InputType::from_letter("s"), InputType::Seconds);
        assert_eq!(InputType::from_letter("m"), InputType::Millis);
        assert_eq!(InputType::from_letter("u"), InputType::Micros);
        assert_eq!(InputType::from_letter("n"), InputType::Nanos);
        assert_eq!(InputType::from_letter("d"), InputType::Discord);
        assert_eq!(InputType::from_letter("t"), InputType::Twitter);
    }

    #[test]
    #[should_panic(expected = "invalid letter for InputType")]
    fn inputtype_from_letter_invalid() {
        let _ = InputType::from_letter("y");  // y tho
    }
}
