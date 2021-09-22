/*
    Too Many Timestamps
    Copyright (c) 2021 0x5c
    SPDX-License-Identifier: LiLiQ-Rplus-1.1
*/

use chrono::Utc;
use clap::ArgMatches;
use crossterm::style::{style, Stylize};

use crate::{
    parsing::{
        parse_timestamp,
        parse_discord,
        parse_twitter,
    },
    types::{
        TimeSource,
        InputType,
        Operation as Op,
    },
};

mod printit;
mod args;
mod types;
mod parsing;

fn main() {
    let current_time_at_start = Utc::now();

    let matches = args::create_app();
    
    let (op, source) = if let true = matches.is_present("timestamp") {
        let it = find_input_type(&matches);
        let input = match parsing::parse_input_to_int(matches.value_of("timestamp").unwrap().into()) {
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

fn find_input_type(matches: &ArgMatches) -> InputType {
    match ["s", "m", "u", "n", "d", "t"].iter().find(|x| (matches).is_present(x)) {
        Some(t) => InputType::from_letter(*t),
        None    => InputType::default(),
    }
}

fn dispatch_parsing(input: i64, input_type: &InputType) -> anyhow::Result<Op> {
    Ok(match input_type {
        InputType::Seconds => Op::Timestamp(parse_timestamp(input, parsing::Split::S)?),
        InputType::Millis  => Op::Timestamp(parse_timestamp(input, parsing::Split::M)?),
        InputType::Micros  => Op::Timestamp(parse_timestamp(input, parsing::Split::U)?),
        InputType::Nanos   => Op::Timestamp(parse_timestamp(input, parsing::Split::N)?),
        InputType::Discord => Op::Discord(parse_discord(input as u64)?),
        InputType::Twitter => Op::Twitter(parse_twitter(input as u64)?),
    })
}
