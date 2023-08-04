use serde::Serialize;

use crate::{error::ParseError, graph_object::GraphObject, meta_data::MetaData};

#[derive(Default, Debug, PartialEq, Serialize)]
pub struct Player {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub stream: Option<String>,
}

impl GraphObject for Player {
    fn prefix() -> &'static str {
        "player"
    }

    fn update_from(&mut self, data: MetaData) -> Result<(), ParseError> {
        match data.tags {
            [] => {
                self.url = data.normalized_url();
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
            ["stream"] => {
                self.stream = Some(data.normalized_url());
            }
            _ => return Err(ParseError::InvalidPropertyTag),
        }
        Ok(())
    }
}
