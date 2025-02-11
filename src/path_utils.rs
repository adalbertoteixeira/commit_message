use log::{debug, error, info};
use serde_json::Value;
use std::process::Command;
use std::str;
use std::{fs, process};

pub fn top_level(directory: &str) {
    let cmd_arg = format!("cd {directory} && git rev-parse --show-toplevel");
    let repo_root_output = Command::new("sh").arg("-c").arg(cmd_arg).output().unwrap();
    debug!("Repository root is {:?}", repo_root_output.status);
    if !repo_root_output.status.success() {
        let error = str::from_utf8(&repo_root_output.stderr).unwrap();
        error!("{:?}", error);
        println!("Path is not valid");
        process::exit(1)
    }
}

// Get current branch
pub fn git_branch(directory: &str) -> String {
    let cmd_arg = format!("cd {directory} && git rev-parse --abbrev-ref HEAD");
    let output = Command::new("sh").arg("-c").arg(cmd_arg).output().unwrap();
    if !output.status.success() {
        let error = str::from_utf8(&output.stderr).unwrap();
        error!("{:?}", error);
        println!("Couldn't get the branch.");
        process::exit(1)
    }
    let current_branch = str::from_utf8(&output.stdout).unwrap().trim();
    info!("Current branch is: {}", current_branch);
    return current_branch.to_owned();
}
// pub fn path_utils() {
//     let mut package_path = "".to_owned();
//     let repo_root_stdout = str::from_utf8(&repo_root_output.stdout)
//         .unwrap()
//         .strip_suffix('\n')
//         .unwrap();
//     package_path.push_str(&repo_root_stdout);
//     package_path.push_str("/package.json");
//     let package_data = fs::read_to_string(package_path).expect("Unable to read file");
//     let parsed_package_data: Value = serde_json::from_str(&package_data).unwrap();
//     debug!("{:?}", &parsed_package_data["config"]["commitizen"]);
//
//     // let package_data = fs::read_to_string(&package_path).expect("Unable to read file");
//     // writeln!(handle, "{:?}", &package_data).unwrap_or_default();
//
//     debug!(
//         "Scopes: {:?}",
//         &parsed_package_data["config"]["commitizen"]["scopes"]
//     );
//
//     let possible_scopes = parsed_package_data["config"]["commitizen"]["scopes"]
//         .as_array()
//         .unwrap();
//
//     let mut possible_scopes_slice: Vec<&str> = vec![];
//     //= &possible_scopes;
//     for item in possible_scopes.iter() {
//         possible_scopes_slice.push(&item.as_str().unwrap());
//     }
//     debug!("{:?}", &possible_scopes_slice);
//
//     debug!("Possible scopes{:?}", &possible_scopes);
//     // let scope_values
//     let possible_types_slice = [
//         "feat", "fix", "docs", "refactor", "test", "build", "ci", "revert", "chore",
//     ];
// }
