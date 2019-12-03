// Copyright 2019 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use clap::{App, Arg, ArgMatches};
use colored::*;
use std::error::Error;
use std::io::Write;
use std::process::Command;
use std::{fs, io, process};
use yaml_rust::yaml::{Yaml, YamlLoader};

#[macro_use]
mod messages;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() {
    let args = App::new("apply-user-defaults")
        .version("0.1.0")
        .author("Michael Sanders <michael.sanders@fastmail.com>")
        .about("Apply macOS user defaults in bulk from YAML file.")
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
            Arg::with_name("FILE")
                .help("Sets the input file to use")
                .required(true),
        )
        .get_matches();

    try_main(args).unwrap_or_else(|err| {
        eprintln!("{}: {}", "Error".red(), err);
        process::exit(1)
    });
}

fn try_main(args: ArgMatches) -> Result<()> {
    let filename = args.value_of("FILE").unwrap();
    let body =
        fs::read_to_string(filename).map_err(|err| format!("Could not open file. {}", err))?;

    messages::set_quiet_output(args.is_present("quiet"));
    messages::set_verbose_output(args.is_present("verbose"));

    let docs = YamlLoader::load_from_str(&body)?;
    let doc = &docs[0];
    let defaults = doc.as_hash().ok_or(format!(
        "Unexpected document type. Expected hash, got {:?}",
        doc
    ))?;
    for (domain, values) in defaults {
        let domain = domain.as_str().ok_or(format!(
            "Unexpected domain value. Expected string, got {:?}",
            domain
        ))?;
        let values = values
            .as_hash()
            .ok_or(format!("Unexpected value. Expected hash, got {:?}", values))?;
        for (key, value) in values {
            let key = key
                .as_str()
                .ok_or(format!("Unexpected value. Expected string, got {:?}", key))?;
            write_default(domain, key, value)?;
        }
    }

    message!("Applied defaults.");
    Ok(())
}

/// Writes the given value as the value for key in domain using the `defaults
/// write` command. Value types are automatically inferred from the YAML enum.
fn write_default(domain: &str, key: &str, value: &Yaml) -> Result<()> {
    let (value_type, value): (&str, std::string::String) = match value {
        Yaml::Real(x) => ("-float", x.to_string()),
        Yaml::Integer(x) => ("-int", x.to_string()),
        Yaml::Boolean(x) => ("-bool", x.to_string()),
        Yaml::String(x) => ("-string", x.to_string()),
        _ => ("", "".to_string()),
    };

    verbose_message!("defaults write {} {} {} {}", domain, key, value_type, value);

    let output = Command::new("defaults")
        .arg("write")
        .arg(domain)
        .arg(key)
        .arg(value_type)
        .arg(value)
        .output()
        .map_err(|err| format!("Failed to invoke defaults command. {}", err))?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    if output.status.success() {
        Ok(())
    } else {
        match output.status.code() {
            Some(code) => process::exit(code),
            None => Err("Process terminated by signal".into()),
        }
    }
}
