use crate::database::connect_to_db::MongoDB;
use crate::models::element_model::Element;
use crate::models::preview_element_model::Preview;
use mongodb::bson;
use mongodb::bson::oid::ObjectId;
use rocket::futures::TryStreamExt;

impl MongoDB {
    pub async fn post_element(
        &self,
        element: String,
        preview: String,
        user_id: String,
    ) -> mongodb::error::Result<()> {
        let collection_element = self.database.collection::<Element>("element");
        let collection_element_preview = self.database.collection::<Preview>("element_preview");

        let id_element = ObjectId::new();

        let element_for_db = Element {
            _id: id_element,
            user_id: user_id.clone(),
            data: element,
        };

        let preview_for_db = Preview {
            _id: id_element,
            user_id: user_id.clone(),
            data: preview,
        };

        collection_element.insert_one(&element_for_db, None).await?;
        collection_element_preview
            .insert_one(&preview_for_db, None)
            .await?;
        Ok(())
    }

    pub async fn get_previews(&self, user_id: String) -> mongodb::error::Result<Vec<Preview>> {
        let collection_element_preview = self.database.collection::<Preview>("element_preview");

        let mut cursor = collection_element_preview
            .find(bson::doc! { "user_id": user_id }, None)
            .await?;

        let mut previews: Vec<Preview> = Vec::new();
        while let Some(preview) = cursor.try_next().await? {
            previews.push(preview)
        }

        Ok(previews)
    }

    pub async fn get_element(&self, element_id: String) -> mongodb::error::Result<Option<Element>> {
        let collection_element = self.database.collection::<Element>("element");

        let element = collection_element
            .find_one(bson::doc! { "_id": element_id }, None)
            .await?;

        Ok(element)
    }
}
