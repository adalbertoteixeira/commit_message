extern crate clap;
use clap::{App, Arg};
use log::{debug, error, info};
use std::io::{self, Write};
use std::process::Command;
use std::str;
extern crate log;
use regex::Regex;

fn main() {
    env_logger::init();
    let matches = App::new("Commit Message Builder")
        .version("1.0")
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .takes_value(true)
                .help("Type of PR")
                .possible_values(&[
                    "feat", "fix", "docs", "refactor", "test", "build", "ci", "revert", "chore",
                ])
                .required(true),
        )
        .arg(
            Arg::with_name("scope")
                .short("s")
                .long("scope")
                .value_name("type")
                .help("Scope of changes")
                .possible_values(&["api", "web", "ee", "send2ben", "test", "tw"])
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .value_name("message")
                .help("Commit message")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("prefix")
                .short("p")
                .long("prefix")
                .value_name("prefix")
                .help("Issue prefix")
                .takes_value(true),
        )
        .get_matches();

    let mut output_string: String = "".to_owned();

    // Add type of PR
    if matches.is_present("type") {
        let pr_type = matches.value_of("type").unwrap();
        output_string.push_str(pr_type);
    }

    // Add scope
    if matches.is_present("scope") {
        let scope = matches.value_of("scope").unwrap();
        output_string.push_str("(");
        output_string.push_str(scope);
        output_string.push_str(")");
    }

    // Get current branch
    let output = Command::new("sh")
        .arg("-c")
        .arg("git rev-parse --abbrev-ref HEAD")
        .output()
        .unwrap();
    debug!("Git branch request is {:?}", output.status);

    if output.status.success() {
        let branch = str::from_utf8(&output.stdout)
            .unwrap()
            .strip_suffix("\n")
            .unwrap();
        info!("Git branch message is {:?}", &branch);

        // Get prefix from param
        if matches.is_present("prefix") {
            let prefix = matches.value_of("prefix").unwrap();
            debug!("Prefix to use is {:?}", prefix);

            let mut raw_regex_string = r"".to_owned();
            raw_regex_string.push_str(prefix);
            let re = Regex::new(&raw_regex_string).unwrap();
            match re.captures(branch) {
                Some(v) => {
                    info!("{:?}", &v);
                    output_string.push_str(": ");
                    output_string.push_str(&v[0]);
                    output_string.push_str(" ");
                }
                None => {
                    println!("NONE!");
                }
            };
        } else {
            output_string.push_str(": ");
            output_string.push_str(branch);
            output_string.push_str(" ");
        };
    } else {
        let error = str::from_utf8(&output.stderr).unwrap();
        error!("{:?}", error);
        return;
    }

    // Add message
    if matches.is_present("message") {
        match matches.value_of("message") {
            Some(message) => {
                output_string.push_str(message);
            }
            None => {
                error!("No scope defined");
                return;
            }
        };
    }
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    writeln!(handle, "{}", output_string).unwrap_or_default(); // add `?` if you care about errors here
}
