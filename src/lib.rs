extern crate clap;
use clap::{App, Arg};
use log::{debug, error, info};
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::str;
extern crate log;
use regex::Regex;
use serde_json::Value;

pub fn commit_message() {
    env_logger::init();
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer

    let repo_root_output = Command::new("sh")
        .arg("-c")
        .arg("git rev-parse --show-toplevel")
        .output()
        .unwrap();
    debug!("Repository root is {:?}", repo_root_output.status);
    if !repo_root_output.status.success() {
        let error = str::from_utf8(&repo_root_output.stderr).unwrap();
        error!("{:?}", error);
        return;
    }

    let mut package_path = "".to_owned();
    let repo_root_stdout = str::from_utf8(&repo_root_output.stdout)
        .unwrap()
        .strip_suffix('\n')
        .unwrap();
    package_path.push_str(&repo_root_stdout);
    package_path.push_str("/package.json");
    let package_data = fs::read_to_string(package_path).expect("Unable to read file");
    let parsed_package_data: Value = serde_json::from_str(&package_data).unwrap();
    debug!("{:?}", &parsed_package_data["config"]["commitizen"]);

    // let package_data = fs::read_to_string(&package_path).expect("Unable to read file");
    // writeln!(handle, "{:?}", &package_data).unwrap_or_default();

    debug!(
        "Scopes: {:?}",
        &parsed_package_data["config"]["commitizen"]["scopes"]
    );

    let possible_scopes = parsed_package_data["config"]["commitizen"]["scopes"]
        .as_array()
        .unwrap();

    let mut possible_scopes_slice: Vec<&str> = vec![];
    //= &possible_scopes;
    for item in possible_scopes.iter() {
        possible_scopes_slice.push(&item.as_str().unwrap());
    }
    debug!("{:?}", &possible_scopes_slice);
    // let possible_scopes_array = Vec::new();
    // for s in &possible_scopes.iter() {
    // possible_scopes_array.push(s.read_to_string());
    // }

    debug!("Possible scopes{:?}", &possible_scopes);
    // let scope_values
    let possible_types_slice = [
        "feat", "fix", "docs", "refactor", "test", "build", "ci", "revert", "chore",
    ];
    let matches = App::new("Commit Message Builder")
        .version("1.0")
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .takes_value(true)
                .help("Type of PR")
                .possible_values(&possible_types_slice)
                .default_value(&possible_types_slice[0]),
        )
        .arg(
            Arg::with_name("scope")
                .short("s")
                .long("scope")
                .value_name("type")
                .help("Scope of changes")
                .possible_values(&possible_scopes_slice)
                .takes_value(true),
            // .default_value(&possible_scopes_slice[0]),
        )
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .value_name("message")
                .help("Commit message")
                .takes_value(true),
            // .required(true),
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

    output_string.push_str(": ");
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
            info!("Prefix to use is {:?}", prefix);

            let mut raw_regex_string = r"".to_owned();
            raw_regex_string.push_str(prefix);
            let re = Regex::new(&raw_regex_string).unwrap();
            info!("Regex to ook for is{:?}", &re);
            match re.captures(branch) {
                Some(v) => {
                    info!("{:?}", &v);
                    output_string.push_str(&v[0]);
                    output_string.push_str(" ");
                }
                None => {
                    println!("NONE!");
                }
            };
        } else {
            // output_string.push_str(": ");
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
    writeln!(handle, "{}", output_string).unwrap_or_default(); // add `?` if you care about errors here
}
