use std::fmt::Display;

pub struct PPCError {
    message: String,
}

impl From<String> for PPCError {
    fn from(value: String) -> Self {
        Self { message: value }
    }
}

impl From<&str> for PPCError {
    fn from(value: &str) -> Self {
        Self { message: String::from(value) }
    }
}

impl Display for PPCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}