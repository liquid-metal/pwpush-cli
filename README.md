pwpush-cli
================================================================================

*Yet another CLI for PasswordPusher*

This repository currently contains an experiment. TBH, mostly I want to try out
things, and get to use some Rust. I use Password Pusher anyways, and I found
that the current CLI tools are fairly incomplete. This tool will be as well for
the forseeable future... but maybe some day :)


## Usage

```
$ pwpush-cli --help
Interact with Password Pusher from the command line

Usage: pwpush-cli [OPTIONS] <COMMAND>

Commands:
  push    Publish a new secret
  expire  Expire an existing secret
  help    Print this message or the help of the given subcommand(s)

Options:
  -u, --url <url>            Password Pusher instance URL. Default is pwpush.com [default: pwpush.com]
  -p, --protocol <protocol>  Password Pusher instance protocol [default: https] [possible values: http, https]
  -e, --email <email>        Email for authenticated requests (goes into X-User-Email header)
  -t, --token <token>        Token for authenticated requests (goes into X-User-Token header)
  -j, --json                 Command output in json. If omitted, human-readable output is produced
  -l, --log <log>            Verbosity of log level. Logs always go to stderr [default: warn] [possible values: error, warn, info, debug, trace]
  -h, --help                 Print help (see more with '--help')
```


## Building

It's rust, uses cargo. If you don't know how to use these you will have problems
in this early stage of the software. As soon as an actually usable binary is
produced, it will probably provided in the form of releases.
