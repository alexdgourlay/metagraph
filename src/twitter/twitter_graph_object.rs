use serde::Serialize;

use crate::{error::ParseError, graph_object::GraphObject};

#[derive(Default, Debug, Serialize)]
pub struct TwitterGraphObject {
    pub card: Option<String>,
    pub site: Option<String>,
    pub creator: Option<String>,
    pub description: Option<String>,
    pub title: Option<String>,
}

impl GraphObject for TwitterGraphObject {
    fn prefix() -> &'static str {
        "twitter"
    }

    fn update_from(&mut self, property_tags: &[&str], content: &str) -> Result<(), ParseError> {
        if let Some(first_tag) = property_tags.first() {
            match *first_tag {
                "card" => {
                    self.card = Some(content.into());
                }
                "site" => {
                    self.site = Some(content.into());
                }
                "creator" => {
                    self.creator = Some(content.into());
                }
                "description" => {
                    self.description = Some(content.into());
                }
                "title" => {
                    self.title = Some(content.into());
                }
                _ => {}
            }
        }
        Ok(())
    }
}