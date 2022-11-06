use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
        }
    }
}

#[derive(Serialize)]
pub struct Page<T> {
    pub message: String,
    pub data: Vec<T>,
    pub page_num: i64,
    pub page_size: i64,
    pub total_elements: i64,
}

impl<T> Page<T> {
    pub fn new(
        message: &str,
        data: Vec<T>,
        page_num: i64,
        page_size: i64,
        total_elements: i64,
    ) -> Page<T> {
        Page {
            message: message.to_string(),
            data,
            page_num,
            page_size,
            total_elements,
        }
    }
}
