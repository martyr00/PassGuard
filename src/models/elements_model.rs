use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Element {
    pub _id: ObjectId,
    pub id_user: String,

    pub name: String,

    pub login: Option<String>,
    pub password: Option<String>,
    pub url: Option<String>,

    pub owners_name: Option<String>,
    pub number: Option<String>,
    pub type_card: Option<String>,
    pub month_card: Option<String>,
    pub year_card: Option<String>,
    pub ccv: Option<String>,

    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub last_name: Option<String>,

    pub company: Option<String>,
    pub mail: Option<String>,
    pub telephone: Option<String>,
    pub address_2: Option<String>,
    pub address_1: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub index: Option<String>,
    pub country: Option<String>,

    pub description: Option<String>,

    pub folder: Option<String>,
    pub favorite: bool,
}