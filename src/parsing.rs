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

pub fn parse_discord(id: u64) -> anyhow::Result<DiscordID> {
    let ts = ((id >> 22) + DISCORD_EPOCH) as i64;
    let (sc, ns) = split_timestamp(ts, 1_000)?;
    let naive = match NaiveDateTime::from_timestamp_opt(sc, ns) {
        Some(n) => n,
        None => panic!("uh oh, discord timestamp out of range? (how???)"),
    };
    let time = DateTime::<Utc>::from_utc(naive, Utc);
    let wid = ((id & 0x3E0000) >> 17) as u8;
    let pid = ((id & 0x1F000) >> 12) as u8;
    let seq = (id & 0xFFF) as u16;
    Ok(DiscordID{time, wid, pid, seq})
}

pub fn parse_twitter(snowflake: u64) -> anyhow::Result<TwitterSnowflake> {
    let ts = ((snowflake >> 22) + TWITTER_EPOCH) as i64;
    let (sc, ns) = split_timestamp(ts, 1_000)?;
    let naive = match NaiveDateTime::from_timestamp_opt(sc, ns) {
        Some(n) => n,
        None => panic!("uh oh, twitter timestamp out of range? (how???)"),
    };
    let time = DateTime::<Utc>::from_utc(naive, Utc);
    let mid = ((snowflake & 0x3FF000) >> 12) as u16;
    let seq = (snowflake & 0xFFF) as u16;
    Ok(TwitterSnowflake{time, mid, seq})
}
