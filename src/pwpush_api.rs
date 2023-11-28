use log::debug;

use crate::{args, errors::PPCError};

pub fn push_text(_ppc_text: &args::PPCText) -> Result<(), PPCError> {
    debug!("start push text");

    if true {
        return Err(PPCError::new("something has gone wrong..."));
    }

    debug!("completed push text normally");
    Ok(())
}