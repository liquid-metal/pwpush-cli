use std::fmt::Display;

use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};

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
    #[arg(id = "email", long, short, requires = "token")]
    pub email: Option<String>,

    /// Token for authenticated requests (goes into X-User-Token header)
    #[arg(id = "token", long, short, requires = "email")]
    pub token: Option<String>,

    /// Command output in json. If omitted, human-readable output is produced
    #[arg(id = "json", long, short, action = ArgAction::SetTrue)]
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

/// Sepcify the object to operate on. These options resemble the three very
/// similar parts of the API.
#[derive(Debug, Subcommand)]
pub enum PPCObject {
    /// Work on text-based secrets (typically passwords)
    Text(PPCText),

    /// Work on files
    File(PPCFile),

    /// Work on URLs
    URL(PPCURL),
}

#[derive(Debug, Args)]
pub struct PPCText {
    /// The URL encoded password or secret text to share
    #[arg(id = "password")]
    pub password_payload: String,

    /// Require recipients to enter this passphrase to view the created push
    #[arg(id = "passphrase", long)]
    pub passphrase: Option<String>,

    /// If authenticated, the URL encoded note for this push. Visible only to the push creator
    #[arg(id = "note", long)]
    pub note: Option<String>,

    /// Expire secret link and delete after this many days
    #[arg(id = "expire-after-days", long)]
    pub expire_after_days: Option<usize>,

    /// Expire secret link and delete after this many views
    #[arg(id = "expire-after-views", long)]
    pub expire_after_views: Option<usize>,

    /// Allow users to delete passwords once retrieved
    #[clap(flatten)]
    pub deletable_by_viewer: Option<DeletableByViewerGroup>,

    /// Helps to avoid chat systems and URL scanners from eating up views
    #[clap(flatten)]
    pub retrieval_step: Option<RetrievalStepGroup>,
}

#[derive(Debug, Args)]
#[group(required = false, multiple = false)]
pub struct DeletableByViewerGroup {
    /// Mutually exclusive with --not-deletable-by-viewer. If not given, instance default is used
    #[arg(id = "is-deletable-by-viewer", long, action = ArgAction::SetTrue)]
    pub is_deletable_by_viewer: bool,

    /// Mutually exclusive with --deletable-by-viewer. If not given, instance default is used
    #[arg(id = "not-deletable-by-viewer", long, action = ArgAction::SetFalse)]
    pub not_deletable_by_viewer: bool,
}

#[derive(Debug, Args)]
#[group(required = false, multiple = false)]
pub struct RetrievalStepGroup {
    /// Mutually exclusive with --without-retrieval-step. If not given, instance default is used
    #[arg(id = "with-retrieval-step", long, action = ArgAction::SetTrue)]
    pub with_retrieval_step: bool,

    /// Mutually exclusive with --with-retrieval-step. If not given, instance default is used
    #[arg(id = "without-retrieval-step", long, action = ArgAction::SetFalse)]
    pub without_retrieval_step: bool,
}

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

impl Display for InstanceProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstanceProtocol::Http => write!(f, "http"),
            InstanceProtocol::Https => write!(f, "https"),
        }
    }
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
