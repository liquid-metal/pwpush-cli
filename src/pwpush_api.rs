use std::fmt;

use log::debug;
use reqwest;
use urlencoding;

use crate::{args, errors::PPCError};

/// Use the password pusher API to publish a single text secret, usually a
/// password. All settings and information are contained in the structs that
/// are given as parameter, and were initially created by clap.
///
/// On error, this function bails out with a `PPCError` with a human-readable
/// message. On success, the output is printed to stdout, either as human-
/// readable text or as JSON if requested with the `-j` option.
pub fn push_text(args: &args::PPCArgs, ppc_text: &args::PPCText) -> Result<(), PPCError> {
    debug!("start push text");

    let client = reqwest::blocking::Client::new();

    // no error handling is needed for the url at this point. the format will
    // always be valid, as the clap parser makes sure that the protocol can only
    // be http or https, and the free-text part in the middle can be virtually
    // anything. If the final URL contains invalid characters or no hostname at
    // all, the URL implementation is solid enough to consume it anyways, and
    // let the user know later in the process what exactly went wrong (e.g.
    // `failed to lookup address` or `empty host`).
    let url = format!("{}://{}/p.json", args.instance_protocol, args.instance_url);
    debug!("URL for request: {}", url);
    let mut builder = client.post(url);

    // only need to check for email or token, as clap ensures that both or none
    // are given. This also means that unwrap() can be used because values are
    // ensured to never be empty.
    if args.email.is_some() {
        builder = builder
            .header("X-User-Email", args.email.as_ref().unwrap())
            .header("X-User-Token", args.token.as_ref().unwrap());
    }

    builder = builder.body(build_body_string(ppc_text));

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

/// Helper function to build the body text with the correct format for the API.
/// For the request bodies, the API does not use a standard format like JSON,
/// but rather some kind of serialized version of a ruby hash, encoded similar
/// to URL parameters.
///
/// Apart from the payload itself, all of the parameters are optional and only
/// included in the request if the user specified them. This is important, as
/// each instance has some server-side defaults, and the user might want to
/// accept these defaults. If we sent all of the parameters for each request,
/// we would not enable users to do so.
///
/// In addition, the API requires us to send strings, notably payload,
/// passphrase and note, in an urlencoded format. The strings in the parameter
/// are plain strings that are encoded on the fly here. We cannot enocde the
/// final resulting string with urlencoding, because this would encode the
/// square brackets as well, but we need them literally. Remember, this string
/// will go into the body, so we encoding things that are explicitly required
/// by the API to be encoded.
///
/// The order of the parameters is not strictly specified and could probably be
/// changed, but tests (see below) can assume that the order of the parameters
/// will be the same as the fields in the `PPCText` struct.
fn build_body_string(ppc_text: &args::PPCText) -> String {
    // at the moment there are 7 possible parameters, so might as well use
    // the idiomatic `with_capacity`.
    // As most of the parameters are optional, we build them in a vec and join
    // them in the end.
    let mut args = Vec::with_capacity(7);

    add_option(&mut args, "payload", &Some(&ppc_text.password_payload));
    add_option(&mut args, "passphrase", &ppc_text.passphrase);
    add_option(&mut args, "note", &ppc_text.note);
    add_option(&mut args, "expire_after_days", &ppc_text.expire_after_days);
    add_option(
        &mut args,
        "expire_after_views",
        &ppc_text.expire_after_views,
    );
    add_option(
        &mut args,
        "deletable_by_viewer",
        &ppc_text.deletable_by_viewer,
    );
    add_option(&mut args, "retrieval_step", &ppc_text.retrieval_step);

    // `join` makes the args into a single string, and we do not have to
    // bother an extra separator at the start or the end of the result.
    let final_body = args.join("&");
    debug!("final body string: {}", final_body);

    final_body
}

/// Helper for the helper - push the contents of the data to the args if data is
/// not None, using the format `password[<key>]=<d>`. Value strings get URL
/// encoded as required by the API.
/// This function is only called for strings that should be urlencoded, for
/// bools and for some numbers (usize), so everything is made into a string and
/// the urlencoding is applied, and nothing should break.
fn add_option<T: fmt::Display>(args: &mut Vec<String>, key: &str, data: &Option<T>) {
    if let Some(d) = data.as_ref() {
        let d = format!("{}", d);
        let formatted = format!(
            "password[{}]={}",
            key,
            &urlencoding::encode(d.as_str()).into_owned()
        );
        debug!("appending `{}` request parameters", formatted);
        args.push(formatted);
    }
}

#[cfg(test)]
mod test {
    use crate::args::PPCText;

    use super::build_body_string;

    // not super useful in practice, but what does the build_body_string
    // function care?
    #[test]
    fn build_body_string_empty_pw() {
        let text = PPCText {
            password_payload: String::from(""),
            passphrase: None,
            note: None,
            expire_after_days: None,
            expire_after_views: None,
            deletable_by_viewer: None,
            retrieval_step: None,
        };

        let actual = build_body_string(&text);
        let expected = String::from("password[payload]=");

        assert_eq!(actual, expected);
    }

    #[test]
    fn build_body_string_simple_pw() {
        let text = PPCText {
            password_payload: String::from("password"),
            passphrase: None,
            note: None,
            expire_after_days: None,
            expire_after_views: None,
            deletable_by_viewer: None,
            retrieval_step: None,
        };

        let actual = build_body_string(&text);
        let expected = String::from("password[payload]=password");

        assert_eq!(actual, expected);
    }

    // see <https://docs.rs/urlencoding/latest/urlencoding/fn.encode.html>
    // for details on the URL encoding. There it explicitly says that some
    // characters (including `_`) are not encoded, so they are also not expected
    // to be here in the test case.
    // Obviously, we do not want to write tests for the urlencoding crate, we
    // just want to make sure that our helper function applies it at the
    // correct places.
    #[test]
    fn build_body_string_pw_urlencoded() {
        let text = PPCText {
            password_payload: String::from("random_§$%&%$_characters with spaces"),
            passphrase: None,
            note: None,
            expire_after_days: None,
            expire_after_views: None,
            deletable_by_viewer: None,
            retrieval_step: None,
        };

        let actual = build_body_string(&text);
        let expected = String::from(
            "password[payload]=random_%C2%A7%24%25%26%25%24_characters%20with%20spaces",
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn build_body_string_pw_with_passphrase() {
        let text = PPCText {
            password_payload: String::from("password"),
            passphrase: Some(String::from("passphrase")),
            note: None,
            expire_after_days: None,
            expire_after_views: None,
            deletable_by_viewer: None,
            retrieval_step: None,
        };

        let actual = build_body_string(&text);
        let expected = String::from("password[payload]=password&password[passphrase]=passphrase");

        assert_eq!(actual, expected);
    }

    #[test]
    fn build_body_string_with_multiple_options() {
        let text = PPCText {
            password_payload: String::from("password"),
            passphrase: Some(String::from("passphrase")),
            note: Some(String::from("this is a note")),
            expire_after_days: Some(5),
            expire_after_views: Some(2),
            deletable_by_viewer: None,
            retrieval_step: None,
        };

        let actual = build_body_string(&text);
        let expected = String::from("password[payload]=password&password[passphrase]=passphrase&\
                                             password[note]=this%20is%20a%20note&password[expire_after_days]=5&\
                                             password[expire_after_views]=2");

        assert_eq!(actual, expected);
    }

    #[test]
    fn build_body_string_with_all_options() {
        let text = PPCText {
            password_payload: String::from("password"),
            passphrase: Some(String::from("passphrase")),
            note: Some(String::from("this is a note")),
            expire_after_days: Some(5),
            expire_after_views: Some(2),
            deletable_by_viewer: Some(true),
            retrieval_step: Some(false),
        };

        let actual = build_body_string(&text);
        let expected = String::from("password[payload]=password&password[passphrase]=passphrase&\
                                             password[note]=this%20is%20a%20note&password[expire_after_days]=5&\
                                             password[expire_after_views]=2&password[deletable_by_viewer]=true&\
                                             password[retrieval_step]=false");

        assert_eq!(actual, expected);
    }
}
