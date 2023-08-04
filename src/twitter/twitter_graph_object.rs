use serde::Serialize;

use crate::{error::ParseError, graph_object::GraphObject, meta_data::MetaData};

use super::property::{Image, Creator, Player};

#[derive(Default, Debug, Serialize)]
pub struct TwitterGraphObject {
    pub card: Option<String>,
    pub site: Option<String>,
    pub creator: Option<Creator>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub image: Option<Image>,
    pub player: Option<Player>,
}

impl GraphObject for TwitterGraphObject {
    fn prefix() -> &'static str {
        "twitter"
    }

    fn update_from(&mut self, data: MetaData) -> Result<(), ParseError> {
        if let Some(first_tag) = data.tags.first() {

            if *first_tag == Image::prefix() {
                let image = self.image.get_or_insert(Image::default());
                image.update_from(data)?;
                return Ok(());
            }

            if *first_tag == Creator::prefix() {
                let creator = self.creator.get_or_insert(Creator::default());
                creator.update_from(data)?;
                return Ok(());
            }

            if *first_tag == Player::prefix() {
                let player = self.player.get_or_insert(Player::default());
                player.update_from(data)?;
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