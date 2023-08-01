use serde::Serialize;

use crate::{graph_object::GraphObject, error::ParseError};

#[derive(Default, Debug, PartialEq, Serialize)]
pub struct Creator {
    pub username: String,
    pub id: Option<String>,
}

impl GraphObject for Creator {
    fn prefix() -> &'static str {
        "creator"
    }

    fn update_from(&mut self, tags: &[&str], content: &str) -> Result<(), ParseError> {
        match tags {
            [] => {
                self.username = content.into();
            }
            ["id"] => {
                self.id = Some(content.into());
            }
            _ => return Err(ParseError::InvalidPropertyTag),
        }
        Ok(())
    }
}
