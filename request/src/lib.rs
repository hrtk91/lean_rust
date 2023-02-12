use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub header: Header,
    pub content: String,
}
