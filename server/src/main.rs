mod tcp_server;
mod cqrs;
mod repositories;
mod commands;
mod models;

use com::packet::Packet;
use com::request::Request;
use com::response::{self, Response};
use models::Post;
use serve::cqrs::commands::Command;
use std::collections::VecDeque;
use std::net::SocketAddr;
use repositories::Repository;

use crate::commands::PostCommand;

pub struct App {
    posts: Repository<Post>,
}

impl App {
    pub fn handle_receive(&mut self, packet: Packet, address: SocketAddr) -> Option<Response> {
        log::trace!("{:?}", packet);

        let req: Request = match serde_json::from_slice(packet.payload.as_slice()) {
            Ok(req) => req,
            Err(err) => {
                log::warn!("failed parse payload {:?} : {:?}", address, err);
                return None;
            }
        };

        match req.header.path.as_str() {
            "/post" => {
                let content = req.content;
                self.post(address, &content)
            }
            "/get" => {
                self.get(address)
            }
            _ => None,
        }
    }

    fn post(&mut self, address: SocketAddr, value: &str) -> Option<Response> {
        log::info!("[{:?}] : {:?}", address, value);

        PostCommand::
            new(address, value.into())
            .exec(&mut self.posts);

        self.get(address)
    }

    fn get(&mut self, address: SocketAddr) -> Option<Response> {
        log::trace!("[{:?}]", address);

        let reversed: Vec<Post> = self.posts.clone().into();
        let content = reversed.iter().map(|x| format!("{}:{}", x.address.to_string(), x.content)).collect::<Vec<String>>();

        Some(response::Response {
            header: response::Header{},
            content: content.join("\n")
        })
    }
}

fn main() {
    env_logger::init();
    log::info!("listen started at 127.0.0.1:8080");
    let mut app = App { posts: VecDeque::new() };
    tcp_server::listen(move |packet: Packet, address: SocketAddr| {
        app.handle_receive(packet, address)
    });
}
