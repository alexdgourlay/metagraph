use serde::Serialize;

use crate::{graph_object::GraphObject, error::ParseError, meta_data::MetaData};

#[derive(Default, Debug, PartialEq, Serialize)]
pub struct Creator {
    pub username: String,
    pub id: Option<String>,
}

impl GraphObject for Creator {
    fn prefix() -> &'static str {
        "creator"
    }

    fn update_from(&mut self, data: MetaData) -> Result<(), ParseError> {
        match data.tags {
            [] => {
                self.username = data.content.into();
            }
            ["id"] => {
                self.id = Some(data.content.into());
            }
            _ => return Err(ParseError::InvalidPropertyTag),
        }
        Ok(())
    }
}
