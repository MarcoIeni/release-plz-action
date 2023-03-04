use std::process::Command;

use crate::ACTION_YML_PATH;

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

fn new_release_plz_line(latest_release: &str) -> String {
    format!("    default: \"{}\"", latest_release)
}

fn new_cargo_semver_checks_line() -> String {
    let cargo_semver_checks_tag = latest_release("obi1kenobi/cargo-semver-checks");
    format!("        tag: {}", cargo_semver_checks_tag)
}

pub fn update_action_yml(release_plz_tag: &str) {
    let mut action_yml = std::fs::read_to_string(ACTION_YML_PATH).unwrap();
    let release_plz_line = release_plz_line(&action_yml).unwrap();
    action_yml = action_yml.replace(&release_plz_line, &new_release_plz_line(&release_plz_tag));
    let cargo_semver_checks_line = cargo_semver_checks_line(&action_yml).unwrap();
    action_yml = action_yml.replace(&cargo_semver_checks_line, &&new_cargo_semver_checks_line());
    std::fs::write(ACTION_YML_PATH, action_yml).unwrap();
}

pub fn create_pr(release_plz_tag: &str) {
    let commit_msg = format!("Update to {}", release_plz_tag);
    let branch = format!("update-{release_plz_tag}");
    Command::new("git")
        .args(&["checkout", "-b", &branch])
        .output()
        .unwrap();
    Command::new("git")
        .args(&["add", ACTION_YML_PATH])
        .output()
        .unwrap();
    Command::new("git")
        .args(&["commit", "-m", &commit_msg])
        .output()
        .unwrap();
    Command::new("git")
        .args(&["push", "origin", &branch])
        .output()
        .unwrap();

    let output = Command::new("gh")
        .args(&[
            "pr",
            "create",
            "--fill",
            "--repo",
            "MarcoIeni/release-plz-action",
        ])
        .output()
        .unwrap();

    Command::new("git")
        .args(&["checkout", "-"])
        .output()
        .unwrap();

    println!("{}", String::from_utf8(output.stdout).unwrap());
}
