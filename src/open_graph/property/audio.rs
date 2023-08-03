use serde::Serialize;

use crate::{error::ParseError, meta_data::MetaData, graph_object::GraphObject};

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

    fn update_from(&mut self, data: MetaData) -> Result<(), ParseError> {
        match data.tags {
            [] | ["url"] => {
                self.url = data.normalized_url();
            }
            ["secure_url"] => {
                self.secure_url = Some(data.content.into());
            }
            ["type"] => {
                self.media_type = Some(data.content.into());
            }
            _ => return Err(ParseError::InvalidPropertyTag),
        }
        Ok(())
    }
}
