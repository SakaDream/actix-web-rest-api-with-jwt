pub mod login_history;
pub mod pagination;
pub mod person;
pub mod response;
pub mod user;
pub mod user_token;

#[derive(Serialize)]
pub struct Page<T> {
    data: Vec<T>,
    curr_page_num: i64,
    page_size: i64,
    total_elements: i64,
}

#[derive(Deserialize)]
pub struct PersonFilter {
    pub name: Option<String>,
    pub gender: Option<String>,
    pub age: Option<i32>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
}
