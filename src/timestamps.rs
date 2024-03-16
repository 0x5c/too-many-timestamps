/*
    Too Many Timestamps
    Copyright (c) 2021 0x5c
    SPDX-License-Identifier: LiLiQ-Rplus-1.1
*/

use chrono::Utc;
use crossterm::style::{style, Stylize};

use crate::{
    parsing::parse_timestamp,
    types::{
        TimeSource,
        InputType,
        Operation as Op,
        DiscordID,
        TwitterSnowflake,
    },
};

mod printit;
mod args;
mod types;
mod parsing;

fn main() {
    let current_time_at_start = Utc::now();

    let matches = args::create_app();
    
    let (op, source) = if let true = matches.contains_id("timestamp") {
        let it = args::find_input_type(&matches);
        let input = match parsing::parse_input_to_int(matches.get_one::<String>("timestamp").unwrap().into()) {
            Ok(t) => t,
            Err(_) => {
                eprintln!("{}", "Error: invalid input (digits only, '.,' allowed but ignored)".red());
                std::process::exit(1);
            },
        };
        let op = match dispatch_parsing(input, &it) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("{} {}", "Error:".red(), style(e).red());
                std::process::exit(1);
            }
        };
        (op, TimeSource::Input(it))
    } else {
        (Op::Timestamp(current_time_at_start), TimeSource::Current)
    };

    printit::printit(op, source);
}

fn dispatch_parsing(input: i64, input_type: &InputType) -> anyhow::Result<Op> {
    Ok(match input_type {
        InputType::Seconds => Op::Timestamp(parse_timestamp(input, parsing::Split::S)?),
        InputType::Millis  => Op::Timestamp(parse_timestamp(input, parsing::Split::M)?),
        InputType::Micros  => Op::Timestamp(parse_timestamp(input, parsing::Split::U)?),
        InputType::Nanos   => Op::Timestamp(parse_timestamp(input, parsing::Split::N)?),
        InputType::Discord => Op::Discord(DiscordID::from_int(input as u64)?),
        InputType::Twitter => Op::Twitter(TwitterSnowflake::from_int(input as u64)?),
    })
}
