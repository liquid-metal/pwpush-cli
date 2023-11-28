//! **Yet another CLI for Password Pusher**
//!
//! Public instance available at <https://pwpush.com>
//!
//! ## CLI interface design
//!
//! The CLI interface needs to provide an ergonomic interface for interactive
//! use as well as in non-interactive environments. This holds true both for
//! input, control and configuration, and output.
//!
//! The interface is structured into subcommands that give a verbose interface
//! with easy-to-remember syntax. The `--help` switch must always be availble.
//! The current subcommands resemble the API quite closely, but more abstract
//! commands are planned in order adjust to actual use cases like searching
//! or bulk publishing. These advanced commands will rely on the functionality
//! of the more basic subcommands, and exporting the basic ones does not hurt
//! the user experience.
//!
//! Due to the design of the API we need to specify the object under operation
//! explicitly with each api call. This results in all of the subcommands being
//! divided into tree sub-subcommands, one for passwords (text), files, and urls
//! respectively.
//!
//! Basic subcommands:
//!   - push -> used to publish a new secretcalls the POST endpoint
//!   - expire
//!   - info
//!   - preview
//!   - audit
//!   - list
//!     - active
//!     - expired
//!
//! The application honors a strict separation of error messages to stderr and
//! normal output to stdout.
//!
//! For output in a machine-readable format, JSON output is planned but not
//! available at the moment.
//!
//! ## API description
//!
//! See <https://pwpush.com/api> for publicly available api documentation.
//!
//! In essence, the API can be divided into three almost identical sections.
//! These are management for file pushes, text pushes and URL pushes. At the
//! moment, following endpoints are defined for each of these:
//!   - GET :url_token
//!   - POST
//!   - GET :url_token/preview
//!   - GET :url_token/audit
//!   - DELETE :url_token
//!   - GET active
//!   - GET expired
//!
//! Each of the endpoints (except the POST endpoint) is prefixed with a single
//! letter that resembles the type of object the endpoint should operate on:
//!   - file pushes -> 'f'
//!   - text pushes -> 'p'
//!   - url pushes -> 'r'
//!
//! Instead of HTTP headers, the API currently relies on a '.json' suffix in
//! order to deliver a JSON response instead of a web page. There is no general
//! api prefix, no api versioning, and the api endpoints reside outside of the
//! localization of 'Password Pusher'. So a final API URL to retrieve all active
//! file pushes in the public pwpush.com instance looks like this:
//! `https://pwpush.com/f/active.json`
//!
//! The API supports authentication in the form of authentication tokens that
//! are bound to a specific user with a login. Authenticated API requests need
//! two special HTTP headers:
//!   - `X-User-Email`: email of the account that the token was generated from
//!   - `X-User-Token`: token out of the accounts token view

mod args;
mod errors;
mod pwpush_api;

use clap::Parser;
use errors::PPCError;
use log::{debug, error, info};

use crate::args::*;

fn main() {
    let args = PPCArgs::parse();

    initialize_logging(&args);

    info!("starting application");

    let res = run(&args);

    if let Err(e) = res {
        error!("task could not complete sucessfully: {}", e);
    } else {
        info!("application terminated normally");
    }
}

fn run(args: &PPCArgs) -> Result<(), PPCError> {
    match &args.action {
        PPCAction::Push(push_command) => match push_command {
            PPCObject::Text(ppc_text) => pwpush_api::push_text(ppc_text),
            PPCObject::File(_) => todo!(),
            PPCObject::URL(_) => todo!(),
        },
        args::PPCAction::Expire(_) => todo!(),
    }
}

fn initialize_logging(args: &PPCArgs) {
    // log output with macros from the log crate are directed to stderr.
    if let Err(e) = stderrlog::new()
        .module(module_path!())
        .timestamp(stderrlog::Timestamp::Second)
        .verbosity(args.log_verbosity as usize)
        .init()
    {
        eprintln!(
            "error initializing logging backend: {:?}\nOperation will \
             continue normally, but logging output may not be \
             available.",
            e
        );
        return;
    }

    debug!("logging framework set up");
}
