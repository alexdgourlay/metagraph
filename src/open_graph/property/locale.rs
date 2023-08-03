use serde::Serialize;

use crate::{error::ParseError, meta_data::MetaData, graph_object::GraphObject};

#[derive(Default, Debug, Serialize)]
pub struct Locale {
    locale: String,
    alternate: Option<Vec<String>>,
}

impl Locale {
    pub fn new(locale: String) -> Self {
        Self {
            locale,
            alternate: None,
        }
    }
}

impl GraphObject for Locale {
    fn prefix() -> &'static str {
        "locale"
    }

    fn update_from(&mut self, data: MetaData) -> Result<(), ParseError> {
        match data.tags {
            [] => {
                self.locale = data.content.into();
            }
            ["alternate"] => {
                let alternate = self.alternate.get_or_insert_with(|| vec![]);
                alternate.push(data.content.into());
            }
            _ => return Err(ParseError::InvalidPropertyTag),
        }
        Ok(())
    }
}
