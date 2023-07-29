use scraper::{ElementRef, Html, Selector};
use std::error::Error;
use url::Url;

use crate::graph_object::RootGraphObject;

#[derive(Debug)]
pub struct Scraper<TRootObject: RootGraphObject + Default> {
    url: Url,
    document: Html,
    graph_object: TRootObject,
}

impl<TRootObject: RootGraphObject + Default> Scraper<TRootObject> {
    pub fn new(url: &str, html: &str) -> Result<Self, Box<dyn Error>> {
        // Parsing validates the supplied url.
        let url = Url::parse(url)?;

        let document = Html::parse_document(html);

        Ok(Self {
            url,
            document,
            graph_object: TRootObject::default(),
        })
    }

    /// Get the CSS selector for meta elements.
    fn selector() -> Selector {
        let selector = format!(
            r#"head > meta[{}^="{}"]"#,
            TRootObject::attribute(),
            TRootObject::prefix()
        );
        Selector::parse(&selector).unwrap()
    }

    /// Get the property attribute from an element.
    fn get_property<'a>(element: &ElementRef<'a>) -> Option<&'a str> {
        element
            .value()
            .attr(TRootObject::attribute())
            .and_then(|property| {
                if !property.starts_with(TRootObject::prefix()) {
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
    pub fn scrape(mut self) -> TRootObject {
        let selector = Self::selector();
        let elements = self.document.select(&selector);

        for element in elements {
            let property = Self::get_property(&element);
            let content = Self::get_content(&element);

            if let (Some(property), Some(content)) = (property, content) {
                let property_tags: Vec<&str> = property.split(":").collect();
                let result = self.graph_object.from(&property_tags[1..], &content);
            }
        }
        self.graph_object
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::open_graph::{property::Image, OpenGraphObject};
    use crate::twitter::TwitterGraphObject;
    use std::{env, fs, time::Instant};

    fn read_document(relative_path: &str) -> String {
        let document_abs_path = env::current_dir().unwrap().join(relative_path);
        fs::read_to_string(document_abs_path).unwrap()
    }

    fn get_verge_scraper<T: RootGraphObject + Default>() -> Scraper<T> {
        Scraper::new(
            "https://www.theverge.com/",
            &read_document("assets/verge-article.html"),
        )
        .unwrap()
    }

    fn get_bbc_scraper<T: RootGraphObject + Default>() -> Scraper<T> {
        Scraper::new(
            "https://www.bbc.com/",
            &read_document("assets/bbc-article.html"),
        )
        .unwrap()
    }

    // Helper to scrape HTML string.
    fn scrape<T: RootGraphObject + Default>(html: &str) -> T {
        Scraper::new("http://x.com", html).unwrap().scrape()
    }

    #[test]
    fn scrape_article() {
        let scraper = get_bbc_scraper();
        let now = Instant::now();
        let result: OpenGraphObject = scraper.scrape();
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
                        <meta property="og:image:url" content="X" />
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
                url: "X".into(),
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
}
