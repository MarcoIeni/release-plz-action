use std::process::Command;

use clap::Parser;

use crate::args::CliArgs;

mod args;

const ACTION_YML_PATH: &str = "../action.yml";

fn latest_release(repo: &str) -> String {
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

fn release_plz_line(action_yml: &str) -> Option<String> {
    for line in action_yml.lines() {
        if line.starts_with("    default: \"release-plz-v") {
            return Some(line.to_string());
        }
    }
    None
}

fn cargo_semver_checks_line(action_yml: &str) -> Option<String> {
    for line in action_yml.lines() {
        if line.starts_with("        tag: v") {
            return Some(line.to_string());
        }
    }
    None
}

fn new_release_plz_line() -> String {
    let release_plz_tag = latest_release("MarcoIeni/release-plz");
    format!("    default: \"{}\"", release_plz_tag)
}

fn new_cargo_semver_checks_line() -> String {
    let cargo_semver_checks_tag = latest_release("obi1kenobi/cargo-semver-checks");
    format!("        tag: {}", cargo_semver_checks_tag)
}

fn main() {
    let args = CliArgs::parse();
    match args {
        CliArgs {
            command: args::Command::Pr,
        } => {
            // edit line of file action.yml that starts with aaaa
            let mut action_yml = std::fs::read_to_string(ACTION_YML_PATH).unwrap();
            let release_plz_line = release_plz_line(&action_yml).unwrap();
            action_yml = action_yml.replace(&release_plz_line, &new_release_plz_line());
            let cargo_semver_checks_line = cargo_semver_checks_line(&action_yml).unwrap();
            action_yml =
                action_yml.replace(&cargo_semver_checks_line, &&new_cargo_semver_checks_line());
            std::fs::write(ACTION_YML_PATH, action_yml).unwrap();
        }
    }
}
