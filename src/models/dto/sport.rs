use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SportDto {
    pub id: i32,
    pub code: String,
    pub link: String,
    pub name: String,
    pub abbreviation: String,
    pub sort_order: i32,
    pub active_status: bool,
}
