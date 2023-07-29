use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    InvalidPropertyTag,
    InvalidProperty(String),
    InvalidContent(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidPropertyTag => {
                write!(f, "Invalid property tag")
            }
            ParseError::InvalidProperty(property_name) => {
                write!(f, "Invalid property, {}", property_name)
            }
            ParseError::InvalidContent(content) => write!(f, "Invalid content, {}", content),
        }
    }
}
