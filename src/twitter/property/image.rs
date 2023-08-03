use serde::Serialize;

use crate::{error::ParseError, meta_data::MetaData, graph_object::GraphObject};

#[derive(Default, Debug, PartialEq, Serialize)]
pub struct Image {
    pub url: String,
    pub alt: Option<String>,
}

impl GraphObject for Image {
    fn prefix() -> &'static str {
        "image"
    }

    fn update_from(&mut self, data: MetaData) -> Result<(), ParseError> {
        match data.tags {
            [] | ["src"] => {
                self.url = data.normalized_url();
            }
            ["alt"] => {
                self.alt = Some(data.content.into());
            }
            _ => return Err(ParseError::InvalidPropertyTag),
        }
        Ok(())
    }
}
