use crate::database::connect_to_db::MongoDB;
use mongodb::bson::oid::ObjectId;
use crate::models::element_model::Element;
use crate::models::preview_element_model::Preview;

impl MongoDB {
    pub async fn post_element(&self, element:Vec<u8>, preview:Vec<u8>, user_id: String) -> mongodb::error::Result<()> {
        let collection_element = self.database.collection::<Element>("element");
        let collection_element_preview = self.database.collection::<Preview>("element_preview");

        let element_for_db = Element {
            _id: ObjectId::new(),
            user_id: user_id.clone(),
            data: element
        };

        let preview_for_db = Preview {
            _id: ObjectId::new(),
            user_id: user_id.clone(),
            data: preview
        };

        collection_element.insert_one(&element_for_db, None).await?;
        collection_element_preview.insert_one(&preview_for_db, None).await?;
        Ok(())
    }
}
