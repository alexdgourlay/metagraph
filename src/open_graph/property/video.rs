use serde::Serialize;

use crate::{error::ParseError, meta_data::MetaData, graph_object::GraphObject};

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

    fn update_from(&mut self, data: MetaData) -> Result<(), ParseError> {
        match data.tags {
            [] | ["url"] => {
                self.url = data.content.into();
            }
            ["secure_url"] => {
                self.secure_url = Some(data.content.into());
            }
            ["type"] => {
                self.media_type = Some(data.content.into());
            }
            ["width"] => {
                self.width = Some(
                    data.content
                        .parse()
                        .map_err(|_| ParseError::InvalidContent(data.content.into()))?,
                );
            }
            ["height"] => {
                self.height = Some(
                    data.content
                        .parse()
                        .map_err(|_| ParseError::InvalidContent(data.content.into()))?,
                );
            }
            ["alt"] => {
                self.alt = Some(data.content.into());
            }
            _ => return Err(ParseError::InvalidPropertyTag),
        }
        Ok(())
    }
}
