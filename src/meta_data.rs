pub struct MetaData<'a> {
    pub tags: &'a [&'a str],
    pub content: &'a str,
}

impl<'a> MetaData<'a> {
    pub fn next(&self) -> Self {
        Self {
            tags: &self.tags[1..],
            content: self.content,
        }
    }
} 