/*
    Too Many Timestamps
    Copyright (c) 2021-2024 0x5c
    SPDX-License-Identifier: LiLiQ-Rplus-1.1
*/

use std::{
    convert::TryInto,
    num::ParseIntError,
};

use anyhow::bail;
use chrono::DateTime;

use crate::types::DT;

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
    let dt = match DateTime::from_timestamp(s, n) {
        Some(n) => n,
        None => bail!("Timestamp out of range!"),
    };
    Ok(dt)
}

/// Parses the timestamp component out of any ID that follows the Snowflake format
#[macro_export]
macro_rules! parse_snowflake_ts {
    ($snowflake:ident, $epoch:path) => {
        {
            let ts = (($snowflake >> 22) + $epoch) as i64;
            let (sc, ns) = $crate::parsing::split_timestamp(ts, 1_000)?;
            let dt = match chrono::DateTime::from_timestamp(sc, ns) {
                Some(n) => n,
                None => {
                    println!("{}", "uh oh, the timestamp is out of range, which is likely a bug!\nA bug report will be appreciated :)");
                    panic!("invalid timestamp in snowflake")
                },
            };
            dt
        }
    };
}


#[cfg(test)]
mod tests {
    use chrono::DateTime;

    // TODO: parse_input_to_int

    // TODO: split_timestamp

    // TODO: parse_timestamp

    const TWITTER_EPOCH: u64 = 1288834974657;

    #[test]
    fn snowflake_ts_macro() -> anyhow::Result<()> {
        // Actual twitter snowflake found in the wild
        let snowflake: u64 = 1442310554454986761;

        // Directly instanciating a DateTime that should be equivalent
        let dt = DateTime::from_timestamp(1632708607, 673000000).unwrap();

        assert_eq!(parse_snowflake_ts!(snowflake, TWITTER_EPOCH), dt);

        Ok(())
    }
}
