use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use crate::command_args::{CommandOption, ToSocket};

pub trait TcpClient {
    fn send(&self, message: &[u8]) -> Option<packet::Packet>;
}

pub struct Client {
    option: CommandOption,
}

impl Client {
    pub fn new(option: CommandOption) -> Client {
        Client { option }
    }
}

impl TcpClient for Client {
    fn send(&self, payload: &[u8]) -> Option<packet::Packet> {
        log::trace!("connection start to {:?}", self.option);
        let mut remote = TcpStream::connect(self.option.to_sock())
            .expect(&format!("failed connect to {:?}", self.option));
        let ep = remote.peer_addr().expect("failed get endpoint");
        log::trace!("connected : {:?}", ep);

        log::trace!("write started {:?}", payload);
        match remote.write(payload) {
            Ok(_) => {
                log::trace!("write succeeded");
            }
            Err(err) => {
                log::error!("send error: {:?}", err);
                return None;
            }
        }

        let mut buf = [0; 1024];
        if let Err(_) = remote.set_read_timeout(Some(Duration::from_millis(10000))) {
            return None;
        }

        log::trace!("waiting response...");
        let buf = match remote.read(&mut buf) {
            Ok(size) => Ok(buf[0..size].to_vec()),
            Err(err) => {
                log::debug!("failed read {:?}", err);
                Err(())
            }
        };

        log::trace!("received {:?}", buf);
        let result = match buf {
            Ok(buf) => packet::parse(buf.to_vec()),
            Err(_) => Err(()),
        };

        match result {
            Ok(packet) => {
                log::trace!("packet {:?}", packet);
                Some(packet)
            }
            Err(_) => None,
        }
    }
}
