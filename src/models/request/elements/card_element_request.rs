use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CardElementRequest {
    pub name: String,
    pub description: String,

    pub owners_name: Option<String>,
    pub number: Option<String>,
    pub type_card: Option<String>,
    pub month_card: Option<String>,
    pub year_card: Option<String>,
    pub ccv: Option<String>,

    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub last_name: Option<String>,

    pub folder: Option<String>,
    pub favourite: bool,
}
