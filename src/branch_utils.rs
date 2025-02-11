use lazy_static::lazy_static;
use log::debug;
use log::info;
use regex::Regex;

pub fn issue_id(git_branch: &str) -> String {
    debug!("Git branch is {}", git_branch);
    lazy_static! {
        static ref ISSUE_ID_REGEX: Regex = Regex::new(r"\d*").unwrap();
    }
    let Some(caps) = ISSUE_ID_REGEX.captures(git_branch) else {
        info!(
            "Issue id not found in branch name. Returning empty string. Git branch: {}",
            git_branch
        );
        return "".to_owned();
    };
    let id = caps.get(0).unwrap().as_str().parse::<i32>().unwrap();
    return id.to_string();
    // return caps.get(0).unwrap().to_owned();
    // let a = ISSUE_ID_REGEX
    //     .captures("811-add-initial-ui-for-76-inf-services")
    //     .and_then(|c| c.get(1).map(|m| m.as_str()));
    // print!("{:?}\n", a);
    // let r: Vec<&str> = ISSUE_ID_REGEX
    //     .find_iter(git_branch)
    //     .map(|n| n.as_str())
}
