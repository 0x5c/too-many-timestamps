/*
    Too Many Timestamps
    Copyright (c) 2021-2024 0x5c
    SPDX-License-Identifier: LiLiQ-Rplus-1.1
*/

use std::fmt;

use chrono::{
    DateTime,
    Utc,
};

//#[macro_use]
use crate::parse_snowflake_ts;

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

#[derive(Debug, PartialEq)]
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

impl DiscordID {
    pub const EPOCH: u64 = 1420070400000;

    pub fn from_int(id: u64) -> anyhow::Result<Self> {
        let time = parse_snowflake_ts!(id, Self::EPOCH);
        let wid = ((id & 0x3E0000) >> 17) as u8;
        let pid = ((id & 0x1F000) >> 12) as u8;
        let seq = (id & 0xFFF) as u16;
        Ok(Self{time, wid, pid, seq})
    }
}

#[derive(Debug, PartialEq)]
pub struct TwitterSnowflake {
    /// Snowflake creation time
    pub time: DT,
    /// Internal machine ID
    pub mid: u16,
    /// Sequential
    pub seq: u16,
}

impl TwitterSnowflake {
    pub const EPOCH: u64 = 1288834974657;

    pub fn from_int(snowflake: u64) -> anyhow::Result<Self> {
        let time = parse_snowflake_ts!(snowflake, Self::EPOCH);
        let mid = ((snowflake & 0x3FF000) >> 12) as u16;
        let seq = (snowflake & 0xFFF) as u16;
        Ok(TwitterSnowflake{time, mid, seq})
    }
}


#[cfg(test)]
mod tests {
    use crate::parse_snowflake_ts;

    use super::{
        InputType,
        DiscordID,
        TwitterSnowflake,
    };

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

    #[test]
    fn discord_from_int() -> anyhow::Result<()> {
        let id: u64 = 564766093051166729;

        let time = parse_snowflake_ts!(id, DiscordID::EPOCH);
        let wid: u8 = 0;
        let pid: u8 = 0;
        let seq: u16 = 9;

        assert_eq!(DiscordID::from_int(id).unwrap(), DiscordID{time, wid, pid, seq});
        
        Ok(())
    }

    #[test]
    fn twitter_from_int() -> anyhow::Result<()> {
        let snowflake: u64 = 1442468859282288643;

        let time = parse_snowflake_ts!(snowflake, TwitterSnowflake::EPOCH);
        let mid: u16 = 343;
        let seq: u16 = 3;

        assert_eq!(TwitterSnowflake::from_int(snowflake).unwrap(), TwitterSnowflake{time, mid, seq});

        Ok(())
    }
}
