use std::io::Write;

use com::{packet, response, request};
use tcp_client::{Client, TcpClient};

mod command_args;
mod tcp_client;

fn main() {
    env_logger::init();
    let option = command_args::read_cmd_args();

    log::info!("started client");

    let client = Client::new(option);
    let stdin = std::io::stdin();
    loop {
        let mut buf = String::from("");
        print!("> ");
        std::io::stdout().flush().unwrap();
        stdin.read_line(&mut buf).expect("failed to readline");

        post(&client, buf.into());
    }
}

fn post(client: &Client, value: String) -> bool {
    let req = request::Request {
        header: request::Header {
            path: "/post".into(),
        },
        content: value,
    };
    let payload = match serde_json::to_vec(&req) {
        Ok(payload) => payload,
        Err(err) => {
            log::warn!("failed parse message {:?}", err);
            return false;
        }
    };
    let packet = packet::Packet {
        header: packet::Header {
            len: (std::mem::size_of::<u32>() + payload.len()) as u32,
        },
        payload,
    };
    let bytes = packet.to_vec();
    if let Some(packet) = client.send(&bytes) {
        let resp = serde_json::from_slice::<response::Response>(&packet.payload);
        log::info!("{:?}", resp);
        if let Ok(resp) = resp {
            let array = resp
                .content
                .split("\n")
                .into_iter()
                .map(|x| format!("{:?}", x));

            for item in array {
                log::info!("{:?}", item)
            }
        }
    }

    true
}
