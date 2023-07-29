use serde::Serialize;
use std::str::FromStr;

use super::property::{Audio, Determiner, Image, Locale, Video};
use crate::{
    error::ParseError,
    graph_object::{Extend, GraphObject, RootGraphObject},
};

#[derive(Default, Debug, Serialize)]
pub struct OpenGraphObject {
    pub title: Option<String>,
    pub media_type: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub determiner: Option<Determiner>,
    pub site_name: Option<String>,
    pub images: Option<Vec<Image>>,
    pub audio: Option<Vec<Audio>>,
    pub video: Option<Vec<Video>>,
    pub locale: Option<Locale>,
}

impl GraphObject for OpenGraphObject {
    fn prefix() -> &'static str {
        "og"
    }

    fn from(&mut self, property_tags: &[&str], content: &str) -> Result<(), ParseError> {
        if let Some(first_tag) = property_tags.first() {
            if *first_tag == Image::prefix() {
                self.images
                    .extend_or_update_last(&property_tags[1..], content)?;
                return Ok(());
            }

            if *first_tag == Audio::prefix() {
                self.audio
                    .extend_or_update_last(&property_tags[1..], content)?;
                return Ok(());
            }

            if *first_tag == Video::prefix() {
                self.video
                    .extend_or_update_last(&property_tags[1..], content)?;
                return Ok(());
            }

            match *first_tag {
                "title" => {
                    self.title = Some(content.into());
                }
                "type" => {
                    self.media_type = Some(content.into());
                }
                "url" => {
                    self.url = Some(content.into());
                }
                "description" => {
                    self.description = Some(content.into());
                }
                "determiner" => {
                    let determiner = Determiner::from_str(content)
                        .map_err(|_| ParseError::InvalidContent(content.into()))?;
                    self.determiner = Some(determiner);
                }
                "site_name" => {
                    self.site_name = Some(content.into());
                }
                "locale" => {
                    self.locale = Some(Locale::new(content.into()));
                }
                _ => {
                    return Err(ParseError::InvalidPropertyTag)
                }
            }
        }

        Ok(())
    }
}

impl RootGraphObject for OpenGraphObject {
    fn attribute() -> &'static str {
        "property"
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update() {
        let mut graph_object = OpenGraphObject::default();
        graph_object.from(&vec!["title"], "title").unwrap();
        assert!(graph_object.title.is_some());

        graph_object.from(&vec!["url"], "url").unwrap();
        assert!(graph_object.url.is_some());

        graph_object
            .from(&vec!["description"], "description")
            .unwrap();
        assert!(graph_object.description.is_some());

        graph_object
            .from(&vec!["determiner"], "a")
            .unwrap();
        assert!(graph_object.determiner.is_some());

        graph_object.from(&vec!["site_name"], "site_name").unwrap();
        assert!(graph_object.site_name.is_some());

        graph_object.from(&vec!["image"], "image").unwrap();
        assert!(graph_object.images.is_some());

        graph_object.from(&vec!["audio"], "audio").unwrap();
        assert!(graph_object.audio.is_some());

        graph_object.from(&vec!["video"], "video").unwrap();
        assert!(graph_object.video.is_some());

        graph_object.from(&vec!["locale"], "locale").unwrap();
        assert!(graph_object.locale.is_some());
    }
}
