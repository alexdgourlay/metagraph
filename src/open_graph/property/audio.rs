use serde::Serialize;

use crate::{error::ParseError, graph_object::GraphObject};

#[derive(Default, Debug, Serialize)]
pub struct Audio {
    url: String,
    secure_url: Option<String>,
    media_type: Option<String>,
}

impl GraphObject for Audio {
    fn prefix() -> &'static str {
        "audio"
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
            _ => return Err(ParseError::InvalidPropertyTag),
        }
        Ok(())
    }
}
