use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
};

pub trait ToSocket {
    fn to_sock(&self) -> SocketAddr;
}

#[derive(Debug)]
pub struct CommandOption {
    pub host: String,
    pub port: String,
}

impl ToSocket for CommandOption {
    fn to_sock(&self) -> SocketAddr {
        let ip: Vec<u8> = self
            .host
            .clone()
            .split(".")
            .map(|x| u8::from_str_radix(x, 10).unwrap())
            .collect();
        SocketAddr::new(
            std::net::IpAddr::V4(Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3])),
            u16::from_str_radix(&self.port, 10).unwrap(),
        )
    }
}

enum ReadState {
    None,
    HostKey,
    PortKey,
}

pub fn read_cmd_args() -> CommandOption {
    let args: Vec<String> = env::args().collect();
    log::trace!("{:?}", args);

    let mut option = CommandOption {
        host: String::from(""),
        port: String::from(""),
    };

    analyze_args(&mut option, args);

    log::trace!("{:?}", option);

    option
}

fn analyze_args(option: &mut CommandOption, args: Vec<String>) {
    let mut state = ReadState::None;
    for arg in args.iter() {
        match arg.as_str() {
            "-h" => {
                if let ReadState::None = state {
                    state = ReadState::HostKey;
                }
            }
            "-p" => {
                if let ReadState::None = state {
                    state = ReadState::PortKey;
                }
            }
            _ => match state {
                ReadState::HostKey => {
                    state = ReadState::None;
                    option.host = arg.clone();
                }
                ReadState::PortKey => {
                    state = ReadState::None;
                    option.port = arg.clone();
                }
                _ => {
                    continue;
                }
            },
        }
    }
}
