use std::net::SocketAddr;

#[derive(Clone)]
pub struct Post {
    pub address: SocketAddr,
    pub content: String,
}
