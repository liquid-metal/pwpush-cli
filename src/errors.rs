use std::fmt::Display;

pub struct PPCError<'a> {
    message: &'a str,
}

impl<'a> PPCError<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}

impl<'a> Display for PPCError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}