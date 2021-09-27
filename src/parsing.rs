/*
    Too Many Timestamps
    Copyright (c) 2021 0x5c
    SPDX-License-Identifier: LiLiQ-Rplus-1.1
*/

use std::{
    convert::TryInto,
    num::ParseIntError,
};

use anyhow::bail;
use chrono::{
    Utc,
    DateTime,
    NaiveDateTime,
};
use crossterm::style::Stylize;

use crate::types::{
    DT,
    DiscordID,
    TwitterSnowflake,
};

const DISCORD_EPOCH: u64 = 1420070400000;
const TWITTER_EPOCH: u64 = 1288834974657;

pub fn parse_input_to_int(input: String) -> Result<i64, ParseIntError> {
    let filterred: String = input.chars().filter(|x| (x != &'.')||(x != &',')).collect();
    filterred.parse()
}

pub enum Split {
    S,
    M,
    U,
    N,
}

/// Split a timestamp into seconds and nanoseconds, at the divisor
/// The multiplier for the nanoseconds part is inversely proprotional to
/// the divisor, so it is derived from it.
pub fn split_timestamp(ts: i64, div: i64) -> anyhow::Result<(i64, u32)> {
    let mul = 1_000_000_000 / div;
    Ok((ts / div, (ts.rem_euclid(div) * mul).try_into()?))
}

pub fn parse_timestamp(ts: i64, split: Split) -> anyhow::Result<DT> {
    let (s, n) = match split {
        Split::S => (ts, 0),
        Split::M => split_timestamp(ts, 1_000)?,
        Split::U => split_timestamp(ts, 1_000_000)?,
        Split::N => split_timestamp(ts, 1_000_000_000)?,
    };
    let ndt = match NaiveDateTime::from_timestamp_opt(s, n) {
        Some(n) => n,
        None => bail!("Timestamp out of range!"),
    };
    Ok(DateTime::<Utc>::from_utc(ndt, Utc))
}

/// Parses the timestamp component out of any ID that follows the Snowflake format
macro_rules! parse_snowflake_ts {
    ($snowflake:ident, $epoch:ident) => {
        {
            let ts = (($snowflake >> 22) + $epoch) as i64;
            let (sc, ns) = split_timestamp(ts, 1_000)?;
            println!("{}, {}", sc, ns);
            let naive = match NaiveDateTime::from_timestamp_opt(sc, ns) {
                Some(n) => n,
                None => {
                    println!("{}", "uh oh, the timestamp is out of range, which is likely a bug!\nA bug report will be appreciated :)".red());
                    panic!("invalid timestamp in snowflake")
                },
            };
            DateTime::<Utc>::from_utc(naive, Utc)
        }
    };
}

pub fn parse_discord(id: u64) -> anyhow::Result<DiscordID> {
    let time = parse_snowflake_ts!(id, DISCORD_EPOCH);
    let wid = ((id & 0x3E0000) >> 17) as u8;
    let pid = ((id & 0x1F000) >> 12) as u8;
    let seq = (id & 0xFFF) as u16;
    Ok(DiscordID{time, wid, pid, seq})
}

pub fn parse_twitter(snowflake: u64) -> anyhow::Result<TwitterSnowflake> {
    let time = parse_snowflake_ts!(snowflake, TWITTER_EPOCH);
    let mid = ((snowflake & 0x3FF000) >> 12) as u16;
    let seq = (snowflake & 0xFFF) as u16;
    Ok(TwitterSnowflake{time, mid, seq})
}


#[cfg(test)]
mod tests {
    use super::*;

    // parse_input_to_int

    // split_timestamp

    // parse_timestamp

    #[test]
    fn snowflake_ts_macro() -> anyhow::Result<()> {
        // Actual twitter snowflake found in the wild
        let snowflake: u64 = 1442310554454986761;

        // Directly instanciating a DateTime that should be equivalent
        let ndt = NaiveDateTime::from_timestamp_opt(1632708607, 673000000).unwrap();
        let dt = DateTime::<Utc>::from_utc(ndt, Utc);

        // assert_eq!(ps_sf_ts(snowflake, TWITTER_EPOCH).unwrap(), dt);
        assert_eq!(parse_snowflake_ts!(snowflake, TWITTER_EPOCH), dt);

        Ok(())
    }

    #[test]
    fn parse_snowflake_discord() -> anyhow::Result<()> {
        let id: u64 = 564766093051166729;

        let time = parse_snowflake_ts!(id, DISCORD_EPOCH);
        let wid: u8 = 0;
        let pid: u8 = 0;
        let seq: u16 = 9;

        assert_eq!(parse_discord(id).unwrap(), DiscordID{time, wid, pid, seq});
        
        Ok(())
    }
    
    #[test]
    fn parse_snowflake_twitter() -> anyhow::Result<()> {
        let snowflake: u64 = 1442468859282288643;

        let time = parse_snowflake_ts!(snowflake, TWITTER_EPOCH);
        let mid: u16 = 343;
        let seq: u16 = 3;

        assert_eq!(parse_twitter(snowflake).unwrap(), TwitterSnowflake{time, mid, seq});

        Ok(())
    }
}
