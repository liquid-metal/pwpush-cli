use log::debug;
use reqwest;

use crate::{args, errors::PPCError};

pub fn push_text(args: &args::PPCArgs, ppc_text: &args::PPCText) -> Result<(), PPCError> {
    debug!("start push text");

    let client = reqwest::blocking::Client::new();
    let mut builder = client.post(format!(
        "{}://{}/p.json",
        args.instance_protocol, args.instance_url
    ));

    // only need to check for email or token, as clap ensures that both or none
    // are given. This also means that unwrap() can be used because values are
    // ensured to never be empty.
    if !args.email.is_none() {
        builder = builder
            .header("X-User-Email", args.email.as_ref().unwrap())
            .header("X-User-Token", args.token.as_ref().unwrap());
    }

    let mut args = Vec::with_capacity(7);
    args.push(format!("password[payload]={}", ppc_text.password_payload));
    if let Some(passphrase) = ppc_text.passphrase.as_ref() {
        args.push(format!("password[passphrase]={}", passphrase));
    }
    if let Some(note) = ppc_text.note.as_ref() {
        args.push(format!("password[note]={}", note));
    }
    if let Some(days) = ppc_text.expire_after_days.as_ref() {
        args.push(format!("password[expire_after_days]={}", days));
    }
    if let Some(views) = ppc_text.expire_after_views.as_ref() {
        args.push(format!("password[expire_after_views]={}", views));
    }
    // TODO: handle deletable and retrieval step options

    builder = builder.body(args.join("&"));

    match builder.send() {
        Ok(response) => {
            println!("response status: {}", response.status());
            println!("response content: {}", response.text().unwrap());
        }
        Err(e) => return Err(PPCError::from(format!("{}", e).as_str())),
    };

    debug!("completed push text normally");
    Ok(())
}
