extern crate clap;
mod branch_utils;
pub mod path_utils;
pub mod storage;
use clap::{App, Arg};
use log::{debug, error, info, warn};
use std::{
    io::{self, Write},
    process::{self, Command},
    str,
};
use storage::load_branch_config;
pub mod prompts;
extern crate log;
use inquire::Confirm;

fn main() {
    env_logger::init();
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(&stdout); // optional: wrap that handle in a buffer

    let matches = App::new("Commit Message Builder")
        .version("1.0")
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("directory")
                .value_name("directory")
                .takes_value(true)
                .help("Optional directory to start from")
                .default_value("."),
        )
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .takes_value(true)
                .help("Type of PR"), // .possible_values(&possible_types_slice)
                                     // .default_value(&possible_types_slice[0]),
        )
        // .arg(
        //     Arg::with_name("scope")
        //         .short("s")
        //         .long("scope")
        //         .value_name("type")
        //         .help("Scope of changes")
        //         // .possible_values(&possible_scopes_slice)
        //         .takes_value(true),
        //     // .default_value(&possible_scopes_slice[0]),
        // )
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
        .arg(
            Arg::with_name("config_directory")
                .short("c")
                .help("Config directory to store the tools global information.")
                .env("COMMIT_TOOL_CONFIG_DIRECTORY")
                .takes_value(true),
        )
        .get_matches();

    let config_directory_matches = matches.value_of("config_directory").unwrap_or("");
    let config_editor_matches = matches.value_of("editor").unwrap_or("");
    info!("Configured editor is {:?}", config_editor_matches);
    let _ = storage::setup_commit_tool(config_directory_matches, config_editor_matches);
    debug!("Arguments: {:?}", matches);
    let directory = matches.value_of("directory").unwrap_or(".");

    info!("Base directory is {:?}", directory);
    path_utils::top_level(&directory);
    let git_branch = path_utils::git_branch(&directory);
    let team_prefix = "INF";
    let issue_id = branch_utils::issue_id(&git_branch);
    let message_name;
    if matches.is_present("message") {
        message_name = matches
            .value_of("message")
            .unwrap()
            .to_string()
            .to_lowercase();
    } else {
        message_name = branch_utils::branch_name(&git_branch)
            .to_lowercase()
            .chars()
            .take(55)
            .collect::<String>();
    };

    let mut can_build_default_message = true;
    let mut output_text: String = "\n\x1b[1;1mCommit utility\x1b[0m\n".to_owned();
    output_text.push_str("- Working in directory ");
    output_text.push_str(&format!("\x1b[1;1m{}\x1b[0m\n", &directory));
    output_text.push_str("- Git branch is ");
    output_text.push_str(&format!("\x1b[1;1m{}\x1b[0m\n", &git_branch));
    output_text.push_str("- Team prefix is ");
    output_text.push_str(&format!("\x1b[1;1m{}\x1b[0m\n", &team_prefix));
    if !&issue_id.is_empty() {
        output_text.push_str("- Issue id is ");
        output_text.push_str(&format!("\x1b[1;1m{}\x1b[0m\n", &issue_id));
    } else {
        can_build_default_message = false;
        output_text.push_str(&format!("\x1b[1;31m- No issue id found\x1b[0m\n"));
    }
    if !&message_name.is_empty() {
        output_text.push_str("- Message name is ");
        output_text.push_str(&format!("\x1b[1;1m{}\x1b[0m\n", &message_name));
    } else {
        can_build_default_message = false;
        output_text.push_str(&format!("\x1b[1;31m- No message name found\x1b[0m\n"));
    }
    let is_new_branch = storage::setup_branch_env(&git_branch, &directory).unwrap();
    if is_new_branch == false {
        let _ = load_branch_config(&git_branch, directory);
    }
    info!("Is new branch: {}", &is_new_branch);
    let (proposed_type, used_types) =
        branch_utils::find_changed_file_types(directory, &is_new_branch);
    info!("Proposed types: {:?}", &proposed_type);
    if proposed_type.is_none() {
        can_build_default_message = false;
    }
    info!("Used types: {:?}", &used_types);
    writeln!(handle, "{}", output_text).unwrap_or_default();

    warn!("Can build message: {}", &can_build_default_message);
    let _ = handle.flush();
    let mut will_accept_suggested_message = false;
    let mut commit_message = "".to_owned();
    if can_build_default_message {
        let mut proposed_output_string: String = "".to_owned();
        proposed_output_string.push_str(&format!(
            "{}: {} [{}] #{}",
            &proposed_type.clone().unwrap(),
            &message_name,
            &team_prefix,
            &issue_id
        ));
        info!("Will propose default message: {}", &proposed_output_string);
        let mut proposed_ouput_message = "".to_owned();
        proposed_ouput_message
            .push_str("We have enough information to propose the following commit message:\n\n");
        proposed_ouput_message.push_str(&format!("\x1b[1;32m{}\x1b[1;0m", &proposed_output_string));
        proposed_ouput_message.push_str("\n");
        writeln!(handle, "{}", proposed_ouput_message).unwrap_or_default();
        let _ = handle.flush();
        let confimation_prompt = Confirm::new("Do you want to accept the proposed message?")
            .with_default(true)
            .prompt();

        will_accept_suggested_message = match confimation_prompt {
            Ok(selection) => {
                commit_message = proposed_output_string;
                selection
            }

            Err(_) => {
                print!("Did not understand your input.");
                process::exit(1);
            }
        }
    }

    let mut additional_commit_message = vec![];
    if !will_accept_suggested_message {
        let mut output_string: String = "".to_owned();
        let selected_issue_id = prompts::issue_id_prompt(&issue_id);
        let selected_team_prefix = prompts::team_prefix_prompt(&team_prefix);
        let selected_type = prompts::select_types_prompt(&proposed_type);
        info!("Selected type: {}", selected_type);

        let scope_options: Vec<&str> = vec![
            "web: Work related to front end",
            "api: work related to the back end",
            "devops: work related to infrastructure, tools, etc.",
        ];

        let message = prompts::select_message_prompt(&git_branch);
        output_string.push_str(&format!(
            // "\x1b[1;32m{}: {} [{}] #{}\x1b[1;0m",
            "{}: {} [{}] #{}",
            selected_type,
            message.to_lowercase(),
            selected_team_prefix,
            selected_issue_id
        ));
        let additional_message = prompts::select_additional_message_prompt();
        if additional_message.is_some() {
            info!("Additional message: {:?}", &additional_message);
            for line in additional_message.unwrap().split("\n") {
                info!("Additional message line : {}", line);
                additional_commit_message.push(line.to_owned());
            }
        }

        // Add scope
        if matches.is_present("scope") {
            let scope = matches.value_of("scope").unwrap();
            output_string.push_str("(");
            output_string.push_str(scope);
            output_string.push_str(")");
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
        commit_message = output_string.clone();
        if additional_commit_message.len() > 0 {
            output_string.push_str("\x1b[1;32m\n");
            for line in &additional_commit_message {
                output_string.push_str(&format!("\n{}", line));
            }
            output_string.push_str("\x1b[1;0m");
        }
        writeln!(handle, "{}", output_string).unwrap_or_default();
        let _ = handle.flush();
    }

    let mut pr_template = None;
    let pr_template_prompt = Confirm::new("Do you want to build a PR template?")
        .with_default(is_new_branch)
        .prompt();

    match pr_template_prompt {
        Ok(selection) => {
            if selection {
                pr_template = Some(prompts::pr_template_prompt());
                writeln!(handle, "{}", &pr_template.clone().unwrap()).unwrap_or_default();
            }
        }
        Err(_) => {
            print!("Did not understand your input.");
            process::exit(1);
        }
    }

    let will_commit_pr = prompts::commit_pr_prompt();
    if will_commit_pr == true {
        let _ = branch_utils::commit_pr(
            directory,
            &commit_message,
            additional_commit_message.clone(),
            &git_branch,
            &pr_template,
        );
    }
}

#[test]
fn it_works() {
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    let output = main();
    info!("TEST {:?}", output);
    assert(output).contains("")
}
