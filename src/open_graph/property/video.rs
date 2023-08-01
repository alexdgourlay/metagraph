use serde::Serialize;

use crate::{error::ParseError, graph_object::GraphObject};

#[derive(Default, Debug, Serialize)]
pub struct Video {
    url: String,
    secure_url: Option<String>,
    media_type: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    alt: Option<String>,
}

impl GraphObject for Video {
    fn prefix() -> &'static str {
        "video"
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
