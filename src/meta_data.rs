use url::Url;

pub struct MetaData<'a> {
    pub site_url: &'a Url,
    pub tags: &'a [&'a str],
    pub content: &'a str,
}

impl<'a> MetaData<'a> {
    pub fn next(&self) -> Self {
        Self {
            site_url: self.site_url,
            tags: &self.tags[1..],
            content: self.content,
        }
    }

    pub fn normalized_url(&self) -> String {
        let url = Url::parse(self.content);

        if let Err(parse_error) = url {
            if parse_error == url::ParseError::RelativeUrlWithoutBase {
                let full_url = self.site_url.join(self.content);

                if let Ok(full_url) = full_url {
                    return full_url.into();
                }
            }
        }

        return self.content.into();
    }
}
