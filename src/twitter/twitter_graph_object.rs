use serde::Serialize;

use crate::{error::ParseError, graph_object::GraphObject, meta_data::MetaData};

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

    fn update_from(&mut self, data: MetaData) -> Result<(), ParseError> {
        if let Some(first_tag) = data.tags.first() {
            match *first_tag {
                "card" => {
                    self.card = Some(data.content.into());
                }
                "site" => {
                    self.site = Some(data.content.into());
                }
                "creator" => {
                    self.creator = Some(data.content.into());
                }
                "description" => {
                    self.description = Some(data.content.into());
                }
                "title" => {
                    self.title = Some(data.content.into());
                }
                _ => {}
            }
        }
        Ok(())
    }
}