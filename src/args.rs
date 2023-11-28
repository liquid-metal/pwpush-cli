use clap::{Args, Parser, Subcommand, ValueEnum};

/// Interact with Password Pusher from the command line
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

    /// Verbosity of log level. Logs always go to stderr.
    #[arg(id = "log", long, short, default_value = "warn")]
    #[clap(value_enum)]
    pub log_verbosity: LogVerbosity,

    /// Top-level action to perform
    #[clap(subcommand)]
    pub action: PPCAction,
}

/// For each PPC object type there is a common set of actions. These subcommands
/// allow a very human-friendly interface for the CLI.
#[derive(Debug, Subcommand)]
pub enum PPCAction {
    /// Publish a new secret.
    #[clap(subcommand)]
    Push(PPCObject),

    /// Expire an existing secret.
    #[clap(subcommand)]
    Expire(PPCObject),
}

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

/// Limit available instance protocols to a valid protocol.
#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum InstanceProtocol {
    /// API calls use http://
    Http,

    /// API calls use https://
    Https,
}

/// Define values that allow the user to specify the preferred log level.
/// These values correspond to the available options in the log crate, see
/// <https://docs.rs/log/latest/log/> for details.
/// Explicit values are given so it can be passed to stderrlog directly.
#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum LogVerbosity {
    /// Only display fatal messages that the application cannot recover from
    Error = 0,

    /// Display relevant information about unexpected states and responses
    Warn = 1,

    /// Print information about general application execution. Log volume is
    /// is kept low enough to not clutter the output
    Info = 2,

    /// Display various information about program flow and internal data
    /// structures
    Debug = 3,

    /// Flood your shell with garbage
    Trace = 4,
}