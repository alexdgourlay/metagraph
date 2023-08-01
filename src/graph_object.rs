use crate::error::ParseError;

pub trait GraphObject {
    fn prefix() -> &'static str;

    fn should_create_new(tags: &[&str]) -> bool {
        match tags.len() {
            0 => true,
            _ => tags[0] == "url",
        }
    }

    fn update_from(&mut self, tags: &[&str], content: &str) -> Result<(), ParseError>;
}

pub trait Extend {
    fn extend_or_update_last(&mut self, tags: &[&str], content: &str) -> Result<(), ParseError>;
}

impl<TObject: GraphObject + Default> Extend for Vec<TObject> {
    fn extend_or_update_last(&mut self, tags: &[&str], content: &str) -> Result<(), ParseError> {
        if TObject::should_create_new(tags) {
            let mut graph_object = TObject::default();
            graph_object.update_from(tags, content)?;
            self.push(graph_object);
        } else {
            if let Some(graph_object) = self.last_mut() {
                graph_object.update_from(tags, content)?;
            }
        }
        Ok(())
    }
}

impl<TObject: GraphObject + Default> Extend for Option<Vec<TObject>> {
    fn extend_or_update_last(
        &mut self,
        tags: &[&str],
        content: &str,
    ) -> Result<(), ParseError> {
        if self.is_none() && !TObject::should_create_new(tags) {
            return Ok(());
        } else {
            let vector = self.get_or_insert_with(|| vec![]);
            vector.extend_or_update_last(tags, content)?;
            Ok(())
        }
    }
}
