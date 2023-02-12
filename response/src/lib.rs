use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub header: Header,
    pub content: String
}