use std::process::Command;

use clap::Parser;
use next_version::NextVersion;

use crate::args::CliArgs;

mod args;
mod pr;

const ACTION_YML_PATH: &str = "../action.yml";

pub fn latest_release(repo: &str) -> String {
    let release_plz_tag = Command::new("gh")
        .args(&["release", "list", "--limit", "1", "--repo", repo])
        .output()
        .expect("failed to execute process");
    let release_plz_tag = String::from_utf8(release_plz_tag.stdout).unwrap();
    let release_plz_tag = release_plz_tag.trim();
    println!("latest tag: {repo}: `{}`", release_plz_tag);
    release_plz_tag
        .split_whitespace()
        .next()
        .unwrap()
        .to_string()
}

fn next_tag() -> String {
    let mut action_tag = latest_release("MarcoIeni/release-plz-action");
    println!("latest tag: {:?}", action_tag);
    // remove `v`
    action_tag.remove(0);
    let semver: semver::Version = action_tag.parse().unwrap();
    let selection = {
        let next_patch = semver.increment_patch();
        let next_minor = semver.increment_minor();
        let next_major = semver.increment_major();
        let next = inquire::Select::new("Version:", vec![next_patch, next_minor, next_major])
            .prompt()
            .unwrap();
        format!("v{next}")
    };
    println!("next tag: {:?}", selection);
    selection
}

fn create_release(tag: &str) {
    Command::new("git")
        .args(&["tag", "-a", tag, "-m", tag])
        .output()
        .unwrap();

    Command::new("git")
        .args(&["push", "origin", tag])
        .output()
        .unwrap();

    Command::new("gh")
        .args(&["release", "create", tag, "--generate-notes"])
        .output()
        .unwrap();
}

fn main() {
    let args = CliArgs::parse();
    match args.command {
        args::Command::Pr => {
            let release_plz_tag = latest_release("MarcoIeni/release-plz");
            pr::update_action_yml(&release_plz_tag);
            pr::create_pr(&release_plz_tag);
        }
        args::Command::Release => {
            let next_tag = next_tag();
            create_release(&next_tag);
        }
    }
}
