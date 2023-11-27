use clap::{Args, Parser, Subcommand, ValueEnum};

/// This struct contains all data from command line arguments and serves as
/// entry point for the clap parser configuration at the same time. During
/// application runtime, fields of this struct will be accessed at various
/// places to take decisions about functions or behavior based on the user input
/// from the command line.
#[derive(Debug, Parser)]
pub struct PPCArgs {
    /// Password Pusher instance URL. Default is pwpush.com
    #[arg(id = "url", long, short, default_value = "pwpush.com")]
    pub instance_url: String,

    /// Password Pusher instance protocol
    #[arg(id = "protocol", long, short, default_value = "https")]
    #[clap(value_enum)]
    pub instance_protocol: InstanceProtocol,

    /// Email for authenticated requests (goes into X-User-Email header)
    #[arg(id = "email", long, short)]
    pub email: Option<String>,

    /// Token for authenticated requests (goes into X-User-Token header)
    #[arg(id = "token", long, short)]
    pub token: Option<String>,

    /// Command output in json. If omitted, human-readable output is produced
    #[arg(id = "json", long, short, action)]
    pub json_output: bool,

    /// Top-level action to perform
    #[clap(subcommand)]
    pub action: PPCAction,
}

/// Limit available instance protocols to a valid protocol.
#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum InstanceProtocol {
    /// API calls use http://
    Http,

    // API calls use https://
    Https,
}

/// For each PPC object type there is a common set of actions. These subcommands
/// allow a very human-friendly interface for the CLI.
#[derive(Debug, Subcommand)]
pub enum PPCAction {
    /// Publish a new secret.
    Push(PushCommand),

    /// Expire an existing secret.
    Expire(ExpireCommand),
}

#[derive(Debug, Args)]
pub struct PushCommand;

#[derive(Debug, Args)]
pub struct ExpireCommand;

#[derive(Debug, Subcommand)]
pub enum PPCObject {
    Text(PPCText),
    File(PPCFile),
    URL(PPCURL),
}

#[derive(Debug, Args)]
pub struct PPCText {}

#[derive(Debug, Args)]
pub struct PPCFile {}

#[derive(Debug, Args)]
pub struct PPCURL {}
