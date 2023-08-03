use serde::Serialize;
use std::str::FromStr;

use super::property::{Audio, Determiner, Image, Locale, Video};
use crate::{
    error::ParseError,
    graph_object::{Extend, GraphObject},
    meta_data::MetaData,
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

    fn update_from(&mut self, data: MetaData) -> Result<(), ParseError> {
        if let Some(first_tag) = data.tags.first() {
            if *first_tag == Image::prefix() {
                self.images.extend_or_update_last(data.next())?;
                return Ok(());
            }

            if *first_tag == Audio::prefix() {
                self.audio.extend_or_update_last(data.next())?;
                return Ok(());
            }

            if *first_tag == Video::prefix() {
                self.video.extend_or_update_last(data.next())?;
                return Ok(());
            }

            match *first_tag {
                "title" => {
                    self.title = Some(data.content.into());
                }
                "type" => {
                    self.media_type = Some(data.content.into());
                }
                "url" => {
                    self.url = Some(data.content.into());
                }
                "description" => {
                    self.description = Some(data.content.into());
                }
                "determiner" => {
                    let determiner = Determiner::from_str(data.content)
                        .map_err(|_| ParseError::InvalidContent(data.content.into()))?;
                    self.determiner = Some(determiner);
                }
                "site_name" => {
                    self.site_name = Some(data.content.into());
                }
                "locale" => {
                    self.locale = Some(Locale::new(data.content.into()));
                }
                _ => return Err(ParseError::InvalidPropertyTag),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use url::Url;

    use super::*;

    #[test]
    fn update() {
        let mut graph_object = OpenGraphObject::default();
        let site_url = Url::parse("http://x.com").unwrap();

        graph_object
            .update_from(MetaData {
                site_url: &site_url,
                tags: &["title"],
                content: "title",
            })
            .unwrap();
        assert!(graph_object.title.is_some());

        graph_object
            .update_from(MetaData {
                site_url: &site_url,
                tags: &["type"],
                content: "type",
            })
            .unwrap();
        assert!(graph_object.media_type.is_some());

        graph_object
            .update_from(MetaData {
                site_url: &site_url,
                tags: &["url"],
                content: "url",
            })
            .unwrap();
        assert!(graph_object.url.is_some());

        graph_object
            .update_from(MetaData {
                site_url: &site_url,
                tags: &["description"],
                content: "description",
            })
            .unwrap();
        assert!(graph_object.description.is_some());

        graph_object
            .update_from(MetaData {
                site_url: &site_url,
                tags: &["determiner"],
                content: "",
            })
            .unwrap();
        assert!(graph_object.determiner.is_some());

        graph_object
            .update_from(MetaData {
                site_url: &site_url,
                tags: &["site_name"],
                content: "site_name",
            })
            .unwrap();
        assert!(graph_object.site_name.is_some());

        graph_object
            .update_from(MetaData {
                site_url: &site_url,
                tags: &["image"],
                content: "image",
            })
            .unwrap();
        assert!(graph_object.images.is_some());

        graph_object
            .update_from(MetaData {
                site_url: &site_url,
                tags: &["audio"],
                content: "audio",
            })
            .unwrap();
        assert!(graph_object.audio.is_some());

        graph_object
            .update_from(MetaData {
                site_url: &site_url,
                tags: &["video"],
                content: "video",
            })
            .unwrap();
        assert!(graph_object.video.is_some());

        graph_object
            .update_from(MetaData {
                site_url: &site_url,
                tags: &["locale"],
                content: "locale",
            })
            .unwrap();
        assert!(graph_object.locale.is_some());
    }
}
