use serde::Serialize;

use crate::{error::ParseError, graph_object::GraphObject};

#[derive(Default, Debug, PartialEq, Serialize)]
pub struct Image {
    pub url: String,
    pub alt: Option<String>,
}

impl GraphObject for Image {
    fn prefix() -> &'static str {
        "image"
    }

    fn update_from(&mut self, tags: &[&str], content: &str) -> Result<(), ParseError> {
        match tags {
            [] | ["src"] => {
                self.url = content.into();
            }
            ["alt"] => {
                self.alt = Some(content.into());
            }
            _ => return Err(ParseError::InvalidPropertyTag),
        }
        Ok(())
    }
}
