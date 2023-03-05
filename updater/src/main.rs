use std::process::Command;

use clap::Parser;

use crate::args::CliArgs;

mod args;
mod pr;
mod release;

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

fn main() {
    let args = CliArgs::parse();
    match args.command {
        args::Command::Pr => {
            let release_plz_tag = latest_release("MarcoIeni/release-plz");
            pr::update_action_yml(&release_plz_tag);
            pr::create_pr(&release_plz_tag);
        }
        args::Command::Release => {
            release::create_release();
        }
    }
}
