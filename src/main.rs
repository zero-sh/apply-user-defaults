// Copyright 2019 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use clap::ArgMatches;
use colored::*;
use regex::Regex;
use std::error::Error;
use std::io::Write;
use std::process::Command;
use std::{env, fs, io, process};
use yaml_rust::yaml::{Yaml, YamlLoader};

#[macro_use]
mod messages;
mod cli;

#[macro_use]
extern crate lazy_static;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() {
    try_main(cli::build_cli().get_matches()).unwrap_or_else(|err| {
        eprintln!("{}: {}", "Error".red(), err);
        process::exit(1)
    });
}

fn try_main(args: ArgMatches) -> Result<()> {
    let filename = args.value_of("FILE").unwrap();
    let body =
        fs::read_to_string(filename).map_err(|err| format!("Could not open file. {}", err))?;
    let expand_env_enabled = !args.is_present("no-env");

    messages::set_quiet_output(args.is_present("quiet"));
    messages::set_dry_run_output(args.is_present("dry-run"));
    messages::set_verbose_output(args.is_present("verbose"));

    let docs = YamlLoader::load_from_str(&body)?;
    let doc = &docs[0];
    let defaults = doc
        .as_hash()
        .ok_or_else(|| format!("Unexpected document type. Expected hash, got {:?}", doc))?;
    for (domain, values) in defaults {
        let domain = domain
            .as_str()
            .ok_or_else(|| format!("Unexpected domain value. Expected string, got {:?}", domain))?;
        let values = values
            .as_hash()
            .ok_or_else(|| format!("Unexpected value. Expected hash, got {:?}", values))?;
        for (key, value) in values {
            let key = key
                .as_str()
                .ok_or_else(|| format!("Unexpected value. Expected string, got {:?}", key))?;
            write_default(domain, key, value, expand_env_enabled)?;
        }
    }

    if !messages::dry_run_output() {
        message!("Applied defaults.");
    }
    Ok(())
}

/// Writes the given value as the value for key in domain using the `defaults
/// write` command. Value types are automatically inferred from the YAML enum.
///
/// If `expand_env_enabled` is true, environment variables in template string
/// syntax are expanded.
fn write_default(domain: &str, key: &str, value: &Yaml, expand_env_enabled: bool) -> Result<()> {
    let (value_type, value): (&str, std::string::String) = match value {
        Yaml::Real(x) => ("-float", x.to_string()),
        Yaml::Integer(x) => ("-int", x.to_string()),
        Yaml::Boolean(x) => ("-bool", x.to_string()),
        Yaml::String(x) => ("-string", x.to_string()),
        _ => ("", "".to_string()),
    };

    let value = if expand_env_enabled {
        expand_env_template(value.as_str())
    } else {
        value
    };

    command_message!("defaults write {} {} {} {}", domain, key, value_type, value);
    if messages::dry_run_output() {
        Ok(())
    } else {
        let output = Command::new("/usr/bin/defaults")
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
}

/// Expands any template strings in the given string in the form of `${VAR}`
/// with matching environment variables.
fn expand_env_template(body: &str) -> std::string::String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([^\\]|^)(\$\{(\w+)\})").unwrap();
        static ref RE_ESCAPED: Regex = Regex::new(r"(\\(\$\{(\w+)\}))").unwrap();
    }

    let mut output: std::string::String = body.into();
    for cap in RE.captures_iter(body) {
        let outer = &cap.get(2).unwrap();
        let inner = &cap.get(3).unwrap();
        if let Ok(replacement) = env::var(inner.as_str()) {
            output.replace_range(outer.start()..outer.end(), replacement.as_str());
        }
    }
    RE_ESCAPED.replace_all(output.as_str(), "$2").into()
}
