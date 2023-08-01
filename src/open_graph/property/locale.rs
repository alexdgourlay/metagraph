use serde::Serialize;

use crate::{graph_object::GraphObject, error::ParseError};

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
    
    fn update_from(&mut self, tags: &[&str], content: &str) -> Result<(), ParseError> {
        match tags {
            [] => {
                self.locale = content.into();
            }
            ["alternate"] => {
                let alternate = self.alternate.get_or_insert_with(|| vec![]);
                alternate.push(content.into());
            }
            _ => return Err(ParseError::InvalidPropertyTag),
        }
        Ok(())
    }
}
