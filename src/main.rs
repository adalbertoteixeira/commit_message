extern crate clap;
pub mod branch_utils;
pub mod path_utils;
use clap::{App, Arg};
use log::{debug, error, info};
use std::{
    io::{self, Write},
    os::unix::process,
};
extern crate log;
use inquire::{
    formatter::OptionFormatter,
    list_option::ListOption,
    ui::{Color, RenderConfig, Styled},
    validator::{StringValidator, Validation},
    CustomUserError, MultiSelect, Select, Text,
};

fn main() {
    env_logger::init();
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer

    // let possible_scopes_array = Vec::new();
    // for s in &possible_scopes.iter() {
    // possible_scopes_array.push(s.read_to_string());
    // }

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
        .arg(
            Arg::with_name("scope")
                .short("s")
                .long("scope")
                .value_name("type")
                .help("Scope of changes")
                // .possible_values(&possible_scopes_slice)
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

    debug!("Arguments: {:?}", matches);
    let directory = matches.value_of("directory").unwrap_or(".");
    info!("Base directory is {:?}", directory);
    path_utils::top_level(&directory);
    let git_branch = path_utils::git_branch(&directory);
    let team_prefix = "INF";
    let issue_id = branch_utils::issue_id(&git_branch);

    let mut output_text: String = "\n\x1b[1;1mCommit utility\x1b[0m\n".to_owned();
    output_text.push_str("- Working in directory ");
    output_text.push_str(&format!("\x1b[1;1m{}\x1b[0m\n", &directory));
    output_text.push_str("- Git branch is ");
    output_text.push_str(&format!("\x1b[1;1m{}\x1b[0m\n", &git_branch));
    output_text.push_str("- Team prefix is ");
    output_text.push_str(&format!("\x1b[1;1m{}\x1b[0m\n", &team_prefix));
    output_text.push_str("- Issue id is ");
    output_text.push_str(&format!("\x1b[1;1m{}\x1b[0m\n", &issue_id));

    writeln!(handle, "{}", output_text).unwrap_or_default();
    let _ = handle.flush();

    let selected_issue_id_prompt = Text::new("Select issue ID")
        .with_default(&issue_id)
        .prompt();

    let selected_issue_id = match selected_issue_id_prompt {
        Ok(issue_id) => issue_id,
        Err(_) => {
            println!("An error happened when selecting the issue id, try again.");
            std::process::exit(1);
        }
    };

    let selected_team_prefix_prompt = Text::new("Select team prefix")
        .with_default(&team_prefix)
        .prompt();

    let selected_team_prefix = match selected_team_prefix_prompt {
        Ok(team) => team,
        Err(_) => {
            println!("An error happened when selecting the team, try again.");
            std::process::exit(1);
        }
    };

    let type_options: Vec<&str> = vec![
"feat: A new feature",
"fix: A bug fix",
"docs: Documentation only changes",
"style: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)",
"refactor: A code change that neither fixes a bug nor adds a feature",
"perf: A code change that improves performance",
"test: Adding missing tests or correcting existing tests",
"build: Changes that affect the build system or external dependencies (example scopes: gulp, broccoli, npm)",
"ci: Changes to our CI configuration files and scripts (example scopes: Travis, Circle, BrowserStack, SauceLabs)",
"chore: Other changes that don't modify src or test files",
"revert: Reverts a previous commit",
    ];

    fn get_short_type(type_str: &str) -> String {
        let parts = type_str.split(": ").collect::<Vec<&str>>();
        let type_short = match parts.get(0) {
            Some(x) => x,
            None => "Unknown",
        };
        return type_short.to_string();
    }

    let type_formatter: OptionFormatter<&str> = &|i| {
        return get_short_type(i.value);
    };

    let selected_types_propmpt = Select::new("Select change type", type_options)
        .with_formatter(type_formatter)
        .prompt();

    let selected_type = match selected_types_propmpt {
        Ok(type_str) => get_short_type(type_str),
        Err(_) => {
            println!("An error happened when selecting the team, try again.");
            std::process::exit(1);
        }
    };
    info!("Selected type: {}", selected_type);

    let scope_options: Vec<&str> = vec![
        "web: Work related to front end",
        "api: work related to the back end",
        "devops: work related to infrastructure, tools, etc.",
    ];
    // let selected_scope_propmpt = MultiSelect::new("Select scope of this change", scope_options)
    //     .with_formatter(type_formatter)
    //     .prompt();
    //
    // let selected_scope = match selected_scope_propmpt {
    //     Ok(scope_str) => get_short_type(scope_str),
    //     Err(_) => {
    //         println!("An error happened when selecting the team, try again.");
    //         std::process::exit(1);
    //     }
    // };
    // fn description_render_config() -> RenderConfig<'static> {
    //     RenderConfig::default()
    //         .with_canceled_prompt_indicator(Styled::new("<skipped>").with_fg(Color::DarkYellow))
    // }
    // let help_message = format!("Current directory: ");
    let message_prompt = Text::new("Enter commit message")
        // .with_autocomplete(FilePathCompleter::default())
        // .with_formatter(&|submission| {
        //     let char_count = submission.chars().count();
        //     if char_count == 0 {
        //         String::from("<skipped>")
        //     } else if char_count <= 20 {
        //         submission.into()
        //     } else {
        //         let mut substr: String = submission.chars().take(17).collect();
        //         substr.push_str("...");
        //         substr
        //     }
        // })
        // .with_render_config(description_render_config())
        // .with_help_message(&help_message)
        .with_validator(|input: &str| {
            let length = input.chars().count();
            // info!("Commit message length: {}", length);
            if length > 55 {
                Ok(Validation::Invalid(
                    format!(
                        "Commit message limit is 55 characters. You have {}.",
                        length
                    )
                    .into(),
                ))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt();

    let message = match message_prompt {
        Ok(message) => message,
        Err(_) => {
            println!("An error happened when selecting the commit message, try again.");
            std::process::exit(1);
        }
    };
    let mut output_string: String = "".to_owned();
    output_string.push_str(&format!(
        "{}: {} [{}] #{}",
        selected_type,
        message.to_lowercase(),
        selected_team_prefix,
        selected_issue_id
    ));
    // println!("Selected type: {}", selected_type);
    // Add scope
    if matches.is_present("scope") {
        let scope = matches.value_of("scope").unwrap();
        output_string.push_str("(");
        output_string.push_str(scope);
        output_string.push_str(")");
    }

    // Get current branch

    // if output.status.success() {
    //
    //     // Get prefix from param
    //     if matches.is_present("prefix") {
    //         let prefix = matches.value_of("prefix").unwrap();
    //         info!("Prefix to use is {:?}", prefix);
    //
    //         let mut raw_regex_string = r"".to_owned();
    //         raw_regex_string.push_str(prefix);
    //         let re = Regex::new(&raw_regex_string).unwrap();
    //         info!("Regex to ook for is{:?}", &re);
    //         match re.captures(branch) {
    //             Some(v) => {
    //                 info!("{:?}", &v);
    //                 output_string.push_str(&v[0]);
    //                 output_string.push_str(" ");
    //             }
    //             None => {
    //                 println!("NONE!");
    //             }
    //         };
    //     } else {
    //         // output_string.push_str(": ");
    //         output_string.push_str(branch);
    //         output_string.push_str(" ");
    //     };
    // } else {
    //     let error = str::from_utf8(&output.stderr).unwrap();
    //     error!("{:?}", error);
    //     return;
    // }

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

#[test]
fn it_works() {
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    let output = main();
    info!("TEST {:?}", output);
    assert(output).contains("")
}
