use serde::Serialize;

use crate::{error::ParseError, meta_data::MetaData, graph_object::GraphObject};

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
