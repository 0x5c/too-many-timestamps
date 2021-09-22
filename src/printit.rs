/*
    Too Many Timestamps
    Copyright (c) 2021 0x5c
    SPDX-License-Identifier: LiLiQ-Rplus-1.1
*/

use chrono::SecondsFormat;
use crossterm::style::{
    Stylize,
    style
};

use crate::types::{
    DT,
    DiscordID,
    Operation,
    TimeSource,
    TwitterSnowflake
};


pub fn printit(op: Operation, source: TimeSource) {
    match op {
        Operation::Timestamp(time) => print_timestamp(time, source),
        Operation::Discord(id) => print_discord(id, source),
        Operation::Twitter(sf) => print_twitter(sf, source),
    }
}

fn print_timestamp(time: DT, source: TimeSource) {
    println!("{} {}", "Using".dark_green(), style(source).dark_green());
    println!("{}", iso8601(time, SecondsFormat::AutoSi, true));
    println!("{}", rfc2822(time));
    println!("{}", combo_ts_line(time))
}

fn print_discord(id: DiscordID, source: TimeSource) {
    println!("{} {}", "Using".dark_green(), style(source).dark_green());
    println!("{}", iso8601(id.time, SecondsFormat::AutoSi, true));
    println!("{}", rfc2822(id.time));
    println!("{}", combo_ts_line(id.time));
    println!("{} {}", "Discord internal worker ID:".dark_cyan(), style(id.wid).blue());
    println!("{} {}", "Discord internal process ID:".dark_cyan(), style(id.pid).blue());
    println!("{} {}", "Discord internal sequential number:".dark_cyan(), style(id.seq).blue());
}

fn print_twitter(sf: TwitterSnowflake, source: TimeSource) {
    println!("{} {}", "Using".dark_green(), style(source).dark_green());
    println!("{}", iso8601(sf.time, SecondsFormat::AutoSi, true));
    println!("{}", rfc2822(sf.time));
    println!("{}", combo_ts_line(sf.time));
    println!("{} {}", "Twitter internal machine ID:".dark_cyan(), style(sf.mid).blue());
    println!("{} {}", "Twitter internal sequential number:".dark_cyan(), style(sf.seq).blue());
}

fn prepare_combined_ts(time: DT) -> Option<String> {
    let num = format!("{:0>10}", safe_timestamp_nanos(time)?.to_string());
    let (sec, txt) = num.split_at(num.len()-9);
    let (ms, txt2) = txt.split_at(3);
    let (us, ns) = txt2.split_at(3);
    Some(format!("{}{}{}{}", sec.blue(), ms.dark_yellow(), us.red(), ns.dark_magenta()))
}

fn combo_ts_line(time: DT) -> String {
    match prepare_combined_ts(time) {
        Some(ts) => {
            let u = format!("{0}{4}{1}{4}{2}{4}{3}", 
                                        "s".blue(),
                                        "ms".dark_yellow(),
                                        "µs".red(),
                                        "ns".dark_magenta(),
                                        "/".dark_cyan());
            format!("{} {} {}", "Unix timestamp:".dark_cyan(), ts, u)
        },
        None => format!("{} {}","Unix timestamp:".dark_cyan(), "[time out of bounds]".red()),
    }
}

fn iso8601(time: DT, format: SecondsFormat, use_z: bool) -> String {
    format!("{} {}",
            "ISO 8601/RFC 3339:".dark_cyan(),
            time.to_rfc3339_opts(format, use_z).blue()
    )
}

fn rfc2822(time: DT) -> String {
    format!("{} {}", "RFC 2822:".dark_cyan(), time.to_rfc2822().blue())
}

fn safe_timestamp_nanos(dt: DT) -> Option<i64> {
    let as_ns = dt.timestamp().checked_mul(1_000_000_000)?;
    as_ns.checked_add(i64::from(dt.timestamp_subsec_nanos()))
}
