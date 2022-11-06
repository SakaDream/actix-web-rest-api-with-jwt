use serde::Deserialize;

#[derive(Deserialize)]
pub struct PersonFilter {
    pub name: Option<String>,
    pub gender: Option<String>,
    pub age: Option<i32>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
}
