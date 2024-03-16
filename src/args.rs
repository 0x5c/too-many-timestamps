/*
    Too Many Timestamps
    Copyright (c) 2021 0x5c
    SPDX-License-Identifier: LiLiQ-Rplus-1.1
*/

use clap::{
    Command,
    Arg,
    ArgMatches,
    ArgAction,
};

use crate::types::InputType;

pub fn create_app() -> ArgMatches {
    Command::new(clap::crate_name!())
            .version(clap::crate_version!())
            .about(clap::crate_description!())
            .after_help("Copyright 2021 0x5c <dev@0x5c.io>\nReleased under the LiLiQ-Rplus-1.1 licence.")
            .override_usage("\n    timestamps (-h | -V)\n    timestamps [[INPUT TYPE] TIMESTAMP]")
            .args_override_self(true)
            .arg(Arg::new("timestamp")
                .value_name("TIMESTAMP")
                .help("The timestamp to decode. Defaults to current time.")
                .num_args(0..=1))
            .next_help_heading("Input Type")
            .arg(clap::Arg::new("s")
                .short('S')
                .long("seconds")
                .help("Interpret input as seconds (the default)")
                .action(ArgAction::SetTrue)
                .group("type"))
            .arg(clap::Arg::new("m")
                .short('M')
                .long("milliseconds")
                .help("Interpret input as milliseconds")
                .action(ArgAction::SetTrue)
                .group("type"))
            .arg(clap::Arg::new("u")
                .short('U')
                .long("microseconds")
                .help("Interpret input as microseconds")
                .action(ArgAction::SetTrue)
                .group("type"))
            .arg(clap::Arg::new("n")
                .short('N')
                .long("nanoseconds")
                .help("Interpret input as nanoseconds")
                .action(ArgAction::SetTrue)
                .group("type"))
            .arg(clap::Arg::new("d")
                .short('D')
                .long("discord")
                .help("Interpret input as a Discord ID")
                .action(ArgAction::SetTrue)
                .group("type"))
            .arg(clap::Arg::new("t")
                .short('T')
                .long("twitter")
                .help("Interpret input as a Twitter Snowflake")
                .action(ArgAction::SetTrue)
                .group("type"))
            .group(clap::ArgGroup::new("type")
                .requires("timestamp"))
            .get_matches()
} 

pub fn find_input_type(matches: &ArgMatches) -> InputType {
    // Iterate over all input types, find the one that is set true, if any
    let input_type = {
        ["s", "m", "u", "n", "d", "t"]
        .iter()
        .find(
            |x| matches
            .get_one::<bool>(x)
            .expect("oops 47df8")
            .to_owned()
        )
    };

    match input_type {
        Some(t) => InputType::from_letter(*t),
        None    => InputType::default(),
    }
}
