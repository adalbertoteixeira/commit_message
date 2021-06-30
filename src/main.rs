extern crate clap;
use clap::{App, Arg};
use log::{error, info};
use std::process::Command;
use std::str;
extern crate log;

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
        .get_matches();

    let mut output_string: String = "".to_owned();

    // Add type of PR
    if matches.is_present("type") {
        let pr_type = matches.value_of("type").unwrap();
        println!("Hello args: {:?}", matches);
        println!("Hello type: {:?}", &pr_type);
        output_string.push_str(pr_type);
    }

    // Add scope
    if matches.is_present("scope") {
        match matches.value_of("scope") {
            Some(scope) => {
                output_string.push_str("(");
                output_string.push_str(scope);
                output_string.push_str(")");
            }
            None => {
                error!("No scope defined");
                return;
            }
        };
    }
    let output = Command::new("sh")
        .arg("-c")
        .arg("git rev-parse --abbrev-ref HEAD")
        .output()
        .expect("failed to execute process");
    println!("Hello args: #{:?}", output);
    let status = output.status.code().unwrap();
    if !output.status.success() {
        info!("ERROR");
        error!("Erroe");
        // return;
    }
    info!("status: {:?}, {:?}", status, output.status.success());

    let _branch = str::from_utf8(&output.stderr).unwrap();
    // io::stdout().write_all(&output.stdout).unwrap();
    println!("{:?}", output_string);
}
