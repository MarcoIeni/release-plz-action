#[derive(clap::Parser, Debug)]
#[command(about, version, author)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// Create Pr to update release-plz and cargo-semver-checks version.
    Pr,
    /// Release the current version.
    Release,
}
