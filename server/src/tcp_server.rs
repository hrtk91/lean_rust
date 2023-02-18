use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use com::packet::{Packet, self};
use com::response::{Response, self};

pub fn listen<F>(mut cbk: F) -> ()
where
    F: FnMut(Packet, SocketAddr) -> Option<Response>,
{
    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed listen");
    let mut handlers: Vec<JoinHandle<(TcpStream, SocketAddr)>> = vec![
        accept(listener.try_clone().expect("failed clone listener")),
        accept(listener.try_clone().expect("failed clone listener")),
    ];
    let max = handlers.len();

    loop {
        let mut notyet: Vec<JoinHandle<(TcpStream, SocketAddr)>> = vec![];
        while let Some(handler) = handlers.pop() {
            if handler.is_finished() {
                let (mut stream, address) = handler.join().expect("failed join handler");

                log::trace!("Accepted: {:?}", address);

                stream
                    .set_read_timeout(Some(Duration::from_millis(1000)))
                    .unwrap();

                let buf = match read_stream(&mut stream) {
                    Ok(size) => size,
                    Err(err) => {
                        log::trace!("failed read stream {:?}", err);
                        continue;
                    }
                };

                log::trace!("{:?}", buf);

                let packet = match packet::parse(buf) {
                    Ok(packet) => cbk(packet, address),
                    Err(_) => None
                };

                let resp = match packet {
                    Some(resp) => resp,
                    None => Response {
                        header: response::Header {},
                        content: "".into(),
                    }
                };
                let resp = serde_json::to_vec(&resp).expect("failed parse response");
                let packet = packet::Packet {
                    header: packet::Header {
                        len: resp.len() as u32,
                    },
                    payload: resp
                };
                if let Err(err) = stream.write(&packet.to_vec()) {
                    log::warn!("failed wirte {:?}", err);
                };

                if let Err(err) = stream.shutdown(std::net::Shutdown::Both) {
                    log::debug!("failed shutdown {:?}", err);
                }

                log::trace!("disconnected : {:?}", address);

            } else {
                notyet.push(handler);
            }
        }

        // まだ終わっていないスレッドを再度プッシュ
        while let Some(handler) = notyet.pop() {
            handlers.push(handler);
        }

        for _ in 0..(max - handlers.len()) {
            handlers.push(accept(listener.try_clone().expect("failed clone listener")))
        }
    }
}

fn read_stream(stream: &mut TcpStream) -> Result<Vec<u8>, &str> {
    let mut buf: [u8; 1024] = [0; 1024];

    if let Ok(size) = stream.read(&mut buf) {
        let ret = buf[0..size].to_vec();
        log::trace!("received: {:?}", ret);
        Ok(ret)
    } else {
        Err("failed recieved")
    }
}

fn accept(listener: TcpListener) -> JoinHandle<(TcpStream, SocketAddr)> {
    thread::spawn(move || match listener.accept() {
        Ok((stream, address)) => (stream, address),
        Err(err) => {
            log::error!("listen error: {:?}", err);
            panic!();
        }
    })
}

