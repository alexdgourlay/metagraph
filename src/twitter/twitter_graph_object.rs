use serde::Serialize;

use crate::{error::ParseError, graph_object::GraphObject, meta_data::MetaData};

use super::property::{Image, Creator};

#[derive(Default, Debug, Serialize)]
pub struct TwitterGraphObject {
    pub card: Option<String>,
    pub site: Option<String>,
    pub creator: Option<Creator>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub image: Option<Image>,
}

impl GraphObject for TwitterGraphObject {
    fn prefix() -> &'static str {
        "twitter"
    }

    fn update_from(&mut self, data: MetaData) -> Result<(), ParseError> {
        if let Some(first_tag) = data.tags.first() {

            if *first_tag == Image::prefix() {
                let image = self.image.get_or_insert_with(|| Image::default());
                image.update_from(data)?;
                return Ok(());
            }

            if *first_tag == Creator::prefix() {
                let creator = self.creator.get_or_insert(Creator::default());
                creator.update_from(data)?;
                return Ok(());
            }

            match *first_tag {
                "card" => {
                    self.card = Some(data.content.into());
                }
                "site" => {
                    self.site = Some(data.content.into());
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