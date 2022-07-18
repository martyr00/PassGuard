use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Element {
    pub id_user: ObjectId,

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
    //login
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

    //note name && desc

    pub favorite: bool,
}

/* login {
pub name: String,
pub login: String,
pub password: String,
pub url: Option<String>,
} */

/* card {
pub name: String,
pub owner's_name: String,
pub number: String,
pub type_card: String,
pub month_card: String,
pub year_card:String
pub CCV:String
} */

/* personal information {
pub name: String,
pub first_name: String,
pub second_name: String,
pub last_name: String,
pub login: String,
pub company: Option<String>,
pub mail: Option<String>,
pub telephone: Option<String>,
pub address_2: Option<String>,
pub address_1: Option<String>,
pub city: Option<String>,
pub region: Option<String>,
pub index: Option<String>,
pub country: Option<String>,
} */

/* note {
    pub name: String,
    pub description: OptionString,
*/