use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DataElementPersonal {
    pub name: String,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub last_name: Option<String>,
    pub login: Option<String>,
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
    pub favorite: bool,
}
