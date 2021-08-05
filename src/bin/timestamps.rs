#![allow(uncommon_codepoints)]

use core::panic;
use std::convert::TryInto;

use clap::{App, Arg, AppSettings};

use chrono::{
    Utc,
    DateTime,
    naive::NaiveDateTime,
};

use too_many_timestamps::printit;

fn main() {
    let current_time_at_start = Utc::now();

    let _matches = App::new(clap::crate_name!())
                            .version(clap::crate_version!())
                            .author(clap::crate_authors!(", "))
                            .about(clap::crate_description!())
                            .after_help("Copyright 2021 0x5c\nReleased under the BSD-3-Clause licence.\n")
                            .setting(AppSettings::ColoredHelp)
                            .arg(clap::Arg::with_name("type")
                                .short("t")
                                .long("type")
                                .value_name("TYPE")
                                .help("Specifies the type of timestamp to interpret the input as.")
                                .takes_value(true))
                            .arg(Arg::with_name("precision")
                                .short("p")
                                .long("precision")
                                .value_name("PRECISION")
                                .help("The seconds/millis/etc precision to display.\nDefaults to displaying as much as possible.")
                                .takes_value(true))
                            .arg(Arg::with_name("timestamp")
                                .value_name("TIMESTAMP")
                                .help("The timestamp to decode.\nDefaults to current time.")
                                .takes_value(true))
                            .get_matches();
    
    
    let ts: i64 = 789456123;
    let _it = printit::InputType::M;
    let source = printit::TimeSource::Input(_it);

    let dt = match &source {
        printit::TimeSource::Current => {
            current_time_at_start
        },
        printit::TimeSource::Input(t) => {
            let (s, n): (i64, u32) = match t {
                printit::InputType::S => split_timestamp(ts, 1),
                printit::InputType::M => split_timestamp(ts, 1_000),
                printit::InputType::U => split_timestamp(ts, 1_000_000),
                printit::InputType::N => split_timestamp(ts, 1_000_000_000),
                printit::InputType::Discord => panic!("wth no!"),
                printit::InputType::Twitter => panic!("wth no!"),
            };
            let ndt = NaiveDateTime::from_timestamp_opt(s, n).unwrap();
            DateTime::<Utc>::from_utc(ndt, Utc)
        }
    };

    
    printit::print_timestamp(dt, source);
}

/// Split a timestamp into seconds and nanoseconds, at the divisor
/// The multiplier for the nanoseconds part is inversely proprotional to
/// the divisor, so it is derived from it.
fn split_timestamp(ts: i64, div: i64) -> (i64, u32) {
    let mul = 1_000_000_000 / div;
    (ts / div, (ts.rem_euclid(div) * mul).try_into().unwrap())
}
