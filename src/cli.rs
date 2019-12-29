// Copyright 2019 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("preferences")
        .version("0.1.1")
        .author("Michael Sanders <michael.sanders@fastmail.com>")
        .about("Modern alternative to macOS defaults command.")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("apply")
                .about("Apply macOS user defaults in bulk from YAML file.")
                .arg(
                    Arg::with_name("FILE")
                        .help("Sets the input file to use")
                        .required(true),
                )
                .arg(
                    Arg::with_name("dry-run")
                        .short("d")
                        .long("dry-run")
                        .help("Don’t actually run anything, just show what would be done."),
                )
                .arg(
                    Arg::with_name("quiet")
                        .short("q")
                        .long("quiet")
                        .help("Quiet mode: suppress normal output"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Verbose mode: include diagnostic info in output"),
                )
                .arg(
                    Arg::with_name("no-env")
                        .long("no-env")
                        .help("Disable environment variable expansion"),
                ),
        )
}
