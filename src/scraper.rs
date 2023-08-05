use scraper::{ElementRef, Html, Selector};
use std::error::Error;
use url::Url;

use crate::{
    graph_object::GraphObject, meta_data::MetaData, open_graph::OpenGraphObject,
    twitter::TwitterGraphObject,
};

pub trait Scraper {
    type RootGraphObject: GraphObject + Default;

    /// Get the name of the attribute used for properties.
    fn attribute() -> &'static str {
        "property"
    }

    fn extract_head(html: &str) -> &str{
        // Anything before the body element is parsed into the head of the document.
        let end = html.find("<body>").unwrap_or(html.len()); 
        &html[0..end]
    }

    /// Get the CSS selector for meta elements.
    fn selector() -> Selector {
        let selector = format!(
            r#"head > meta[{}^="{}"]"#,
            Self::attribute(),
            Self::RootGraphObject::prefix()
        );
        Selector::parse(&selector).unwrap()
    }

    /// Get the property attribute from an element.
    fn get_property<'a>(element: &ElementRef<'a>) -> Option<&'a str> {
        element
            .value()
            .attr(Self::attribute())
            .and_then(|property| {
                if !property.starts_with(Self::RootGraphObject::prefix()) {
                    return None;
                }
                return Some(property);
            })
    }

    /// Get the content attribute from an element.
    fn get_content<'a>(element: &ElementRef<'a>) -> Option<&'a str> {
        element.value().attr("content")
    }

    /// Scrape the document for properties.
    fn scrape(url: &str, html: &str) -> Result<Self::RootGraphObject, Box<dyn Error>> {
        // Parsing validates the supplied url.
        let url = Url::parse(url)?;

        let head = Self::extract_head(html);
        let document = Html::parse_document(head);

        let selector = Self::selector();
        let elements = document.select(&selector);

        let mut result = Self::RootGraphObject::default();

        for element in elements {
            let property = Self::get_property(&element);
            let content = Self::get_content(&element);

            if let (Some(property), Some(content)) = (property, content) {
                let property_tags: Vec<&str> = property.split(":").collect();

                let meta_data = MetaData {
                    site_url: &url,
                    tags: &property_tags,
                    content,
                };

                let _ = result.update_from(meta_data.next());
            }
        }

        return Ok(result);
    }
}

pub struct OpenGraphScraper {}

impl Scraper for OpenGraphScraper {
    type RootGraphObject = OpenGraphObject;
}

pub struct TwitterScraper {}

impl Scraper for TwitterScraper {
    type RootGraphObject = TwitterGraphObject;

    fn attribute() -> &'static str {
        "name"
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::open_graph::{property::Image, OpenGraphObject};
    use std::{env, fs, time::Instant};

    type TestScraper = OpenGraphScraper;
    type TestGraphObject = OpenGraphObject;

    fn read_document(relative_path: &str) -> String {
        let document_abs_path = env::current_dir().unwrap().join(relative_path);
        fs::read_to_string(document_abs_path).unwrap()
    }

    fn scrape_verge() -> TestGraphObject {
        TestScraper::scrape(
            "https://www.theverge.com/",
            &read_document("assets/verge-article.html"),
        )
        .unwrap()
    }

    fn scrape_bbc() -> TestGraphObject {
        TestScraper::scrape(
            "https://www.bbc.com/",
            &read_document("assets/bbc-article.html"),
        )
        .unwrap()
    }

    // Helper to scrape HTML string.
    fn scrape(html: &str) -> TestGraphObject {
        TestScraper::scrape("http://x.com", html).unwrap()
    }

    #[test]
    fn scrape_article() {
        let now = Instant::now();
        let result = scrape_bbc();
        println!("Elapsed: {:?}", now.elapsed());
        println!("{:#?}", serde_json::to_string(&result).unwrap());
    }

    #[test]
    fn scrape_title_single() {
        let result: OpenGraphObject = scrape(
            r#"<head>
                        <meta property="og:title" content="X" />
                    </head>"#,
        );
        assert_eq!(result.title, Some("X".into()));
    }

    #[test]
    fn scrape_title_multiple() {
        let result: OpenGraphObject = scrape(
            r#"<head>
                        <meta property="og:title" content="X" />
                        <meta property="og:title" content="Y" />
                    </head>"#,
        );
        assert_eq!(result.title, Some("Y".into()), "Should equal last value");
    }

    #[test]
    fn no_scrape_outside_head() {
        let result: OpenGraphObject = scrape(
            r#"<head></head>
            <body>
                <meta property="og:title" content="X" />
            </body>
                    "#,
        );
        assert_eq!(result.title, None);
    }

    #[test]
    fn scrape_image() {
        let result: OpenGraphObject = scrape(
            r#"<head>
                        <meta property="og:image:url" content="http://x.com/image.jpg" />
                        <meta property="og:image:type" content="image/jpeg" />
                        <meta property="og:image:secure_url" content="X" />
                        <meta property="og:image:width" content="10" />
                        <meta property="og:image:height" content="10" />
                        <meta property="og:image:alt" content="alt" />
                    </head>"#,
        );
        assert_eq!(
            result.images,
            Some(vec![Image {
                url: "http://x.com/image.jpg".into(),
                media_type: Some("image/jpeg".into()),
                secure_url: Some("X".into()),
                width: Some(10),
                height: Some(10),
                alt: Some("alt".into()),
            }])
        );
    }

    #[test]
    fn scrape_image_multiple_a() {
        let result: OpenGraphObject = scrape(
            r#"<head>
                        <meta property="og:image" content="X" />
                        <meta property="og:image" content="Y" />
                    </head>"#,
        );
        assert_eq!(result.images.unwrap().len(), 2);
    }

    #[test]
    fn scrape_image_multiple_b() {
        let result: OpenGraphObject = scrape(
            r#"<head>
                        <meta property="og:image" content="X" />
                        <meta property="og:image:url" content="Y" />
                    </head>"#,
        );
        assert_eq!(result.images.unwrap().len(), 2);
    }

    #[test]
    fn scrape_image_no_url() {
        let result: OpenGraphObject = scrape(
            r#"<head>
                        <meta property="og:image:alt" content="X" />
                    </head>"#,
        );
        assert!(result.images.is_none());
    }

    #[test]
    fn scrape_image_relative_url() {
        let result: OpenGraphObject = scrape(
            r#"<head>
                        <meta property="og:image" content="./image.jpg" />
                    </head>"#,
        );
        assert_eq!(
            result.images,
            Some(vec![Image {
                url: "http://x.com/image.jpg".into(),
                ..Image::default()
            }])
        )
    }
}
