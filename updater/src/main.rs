use clap::Parser;

use crate::args::CliArgs;

mod args;
mod pr;

const ACTION_YML_PATH: &str = "../action.yml";

fn main() {
    let args = CliArgs::parse();
    match args.command {
        args::Command::Pr => {
            let release_plz_tag = pr::latest_release("MarcoIeni/release-plz");
            pr::update_action_yml(&release_plz_tag);
            pr::create_pr(&release_plz_tag);
        }
        args::Command::Release => todo!(),
    }
}
