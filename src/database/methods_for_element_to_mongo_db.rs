use crate::database::connect_to_db::MongoDB;
use crate::models::elements_model::Element;

impl MongoDB {
    pub async fn post_element(&self, element: &Element) -> mongodb::error::Result<()> {
        let collection_element = self.database.collection::<Element>("element");

        collection_element.insert_one(&*element, None).await?;
        Ok(())
    }
}
