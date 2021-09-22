/*
    Too Many Timestamps
    Copyright (c) 2021 0x5c
    SPDX-License-Identifier: LiLiQ-Rplus-1.1
*/

use clap::{
    App,
    AppSettings,
    Arg,
    ArgMatches,
};

pub fn create_app() -> ArgMatches {
    App::new(clap::crate_name!())
            .version(clap::crate_version!())
            .about(clap::crate_description!())
            .after_help("Copyright 2021 0x5c <dev@0x5c.io>\nReleased under the LiLiQ-Rplus-1.1 licence.")
            .override_usage("timestamps (-h | -V)\n    timestamps [[INPUT TYPE] TIMESTAMP]")
            .setting(AppSettings::ColoredHelp)
            .setting(AppSettings::AllArgsOverrideSelf)
            .setting(AppSettings::DeriveDisplayOrder)
            .arg(Arg::new("timestamp")
                .value_name("TIMESTAMP")
                .about("The timestamp to decode. Defaults to current time.")
                .takes_value(true))
            .help_heading("INPUT TYPE")
            .arg(clap::Arg::new("s")
                .short('S')
                .long("seconds")
                .about("Interpret input as seconds (the default)")
                .group("type"))
            .arg(clap::Arg::new("m")
                .short('M')
                .long("milliseconds")
                .about("Interpret input as milliseconds")
                .group("type"))
            .arg(clap::Arg::new("u")
                .short('U')
                .long("microseconds")
                .about("Interpret input as microseconds")
                .group("type"))
            .arg(clap::Arg::new("n")
                .short('N')
                .long("nanoseconds")
                .about("Interpret input as nanoseconds")
                .group("type"))
            .arg(clap::Arg::new("d")
                .short('D')
                .long("discord")
                .about("Interpret input as a Discord ID")
                .group("type"))
            .arg(clap::Arg::new("t")
                .short('T')
                .long("twitter")
                .about("Interpret input as a Twitter Snowflake")
                .group("type"))
            .group(clap::ArgGroup::new("type")
                .requires("timestamp"))
            .get_matches()
} 
