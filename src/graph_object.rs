use crate::{error::ParseError, meta_data::MetaData};

pub trait GraphObject {

    fn prefix() -> &'static str;

    fn should_create_new(tags: &[&str]) -> bool {
        match tags.len() {
            0 => true,
            _ => tags[0] == "url",
        }
    }

    fn update_from(&mut self, data: MetaData) -> Result<(), ParseError>;
}

pub trait Update {
    fn extend_or_update_last(&mut self, data: MetaData) -> Result<(), ParseError>;
}

impl<TObject: GraphObject + Default> Update for Vec<TObject> {
    fn extend_or_update_last(&mut self, data: MetaData) -> Result<(), ParseError> {
        if TObject::should_create_new(data.tags) {
            let mut graph_object = TObject::default();
            graph_object.update_from(data)?;
            self.push(graph_object);
        } else {
            if let Some(graph_object) = self.last_mut() {
                graph_object.update_from(data)?;
            }
        }
        Ok(())
    }
}

impl<TObject: GraphObject + Default> Update for Option<Vec<TObject>> {
    fn extend_or_update_last(&mut self, data: MetaData) -> Result<(), ParseError> {
        if self.is_none() && !TObject::should_create_new(data.tags) {
            return Ok(());
        } else {
            let vector = self.get_or_insert(vec![]);
            vector.extend_or_update_last(data)?;
            Ok(())
        }
    }
}
