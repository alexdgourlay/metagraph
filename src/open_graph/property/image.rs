use serde::Serialize;

use crate::{error::ParseError, graph_object::GraphObject};

#[derive(Default, Debug, PartialEq, Serialize)]
pub struct Image {
    pub url: String,
    pub secure_url: Option<String>,
    pub media_type: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub alt: Option<String>,
}

impl GraphObject for Image {
    fn prefix() -> &'static str {
        "image"
    }

    fn update_from(&mut self, tags: &[&str], content: &str) -> Result<(), ParseError> {
        match tags {
            [] | ["url"] => {
                self.url = content.into();
            }
            ["secure_url"] => {
                self.secure_url = Some(content.into());
            }
            ["type"] => {
                self.media_type = Some(content.into());
            }
            ["width"] => {
                self.width = Some(
                    content
                        .parse()
                        .map_err(|_| ParseError::InvalidContent(content.into()))?,
                );
            }
            ["height"] => {
                self.height = Some(
                    content
                        .parse()
                        .map_err(|_| ParseError::InvalidContent(content.into()))?,
                );
            }
            ["alt"] => {
                self.alt = Some(content.into());
            }
            _ => return Err(ParseError::InvalidPropertyTag),
        }
        Ok(())
    }
}
