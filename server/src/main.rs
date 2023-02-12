use packet::Packet;
use request::Request;
use std::net::SocketAddr;

mod tcp;

pub struct App {
    requests: Vec<(SocketAddr, String)>,
}

impl App {
    pub fn handle_receive(&mut self, packet: Packet, address: SocketAddr) {
        log::trace!("{:?}", packet);

        let req: Request = match serde_json::from_slice(packet.payload.as_slice()) {
            Ok(req) => req,
            Err(err) => {
                log::warn!("failed parse payload {:?} : {:?}", address, err);
                return;
            }
        };

        match req.header.path.as_str() {
            "/post" => {
                let content = req.content;
                self.post(address, &content);
            }
            "/get" => {
                let content = req.content;
                self.get(address);
            }
            _ => (),
        }
    }

    fn post(&mut self, address: SocketAddr, value: &str) {
        log::info!("[{:?}] : {:?}", address, value);

        if self.requests.len() > 50 {
            self.requests.pop();
        }

        self.requests.push((address, value.into()))
    }

    fn get(&mut self, address: SocketAddr) {
        log::trace!("[{:?}]", address);

        let reversed = self.requests.reverse();
    }
}

fn main() {
    env_logger::init();
    log::info!("listen started at 127.0.0.1:8080");
    let mut app = App { requests: vec![] };
    tcp::listen(move |packet: Packet, address: SocketAddr| app.handle_receive(packet, address));
}
