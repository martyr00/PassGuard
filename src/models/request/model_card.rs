use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataElementCard {
    pub name: String,
    pub owners_name: Option<String>,
    pub number: Option<String>,
    pub type_card: Option<String>,
    pub month_card: Option<String>,
    pub year_card: Option<String>,
    pub ccv: Option<String>,

    pub description: Option<String>,
    pub favorite: bool,
}
