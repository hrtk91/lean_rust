use std::{net::TcpStream, io::Write};

use crate::command_args::{CommandOption, ToSocket};

pub trait TcpClient {
    fn send(&self, message: &str) -> Result<(), std::io::Error>;
}

pub struct Client {
    option: CommandOption,
}

impl Client {
    pub fn new(option: CommandOption) -> Client {
        Client {
            option,
        }
    }

}

impl TcpClient for Client {
    fn send(&self, message: &str) -> Result<(), std::io::Error> {
        log::trace!("connection start to {:?}", self.option);
        let mut remote = TcpStream::connect(self.option.to_sock()).expect(&format!("failed connect to {:?}", self.option));
        let ep = remote.peer_addr().expect("failed get endpoint");
        log::trace!("connected : {:?}", ep);

        match remote.write(message.as_bytes()) {
            Ok(_) => {
                remote.shutdown(std::net::Shutdown::Both).expect(&format!("failed shutdown {:?}", self.option));
                Ok(())
            },
            Err(err) => {
                log::error!("send error: {:?}", err);
                Err(err)
            }
        }
    }
}
