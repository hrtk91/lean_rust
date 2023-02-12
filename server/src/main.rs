use com::packet::Packet;
use com::request::Request;
use com::response::{self, Response};
use std::net::SocketAddr;

mod tcp_server;

pub struct App {
    requests: Vec<(SocketAddr, String)>,
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

        if self.requests.len() > 50 {
            self.requests.pop();
        }

        self.requests.push((address, value.into()));

        self.get(address)
    }

    fn get(&mut self, address: SocketAddr) -> Option<Response> {
        log::trace!("[{:?}]", address);

        let mut requests = self.requests.clone();
        requests.reverse();
        let content = requests.iter().map(|x| format!("{}:{}", x.0.to_string(), x.1)).collect::<Vec<String>>();

        Some(response::Response {
            header: response::Header{},
            content: content.join("\n")
        })
    }
}

fn main() {
    env_logger::init();
    log::info!("listen started at 127.0.0.1:8080");
    let mut app = App { requests: vec![] };
    tcp_server::listen(move |packet: Packet, address: SocketAddr| {

        app.handle_receive(packet, address)
    });
}
