use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread::{self, JoinHandle};

fn handle_stream(stream: &mut TcpStream) -> [u8; 1024] {
    // stream.set_read_timeout(Some(Duration::new(1, 0))).expect("failed set timeout");
    let mut buf: [u8; 1024] = [0; 1024];
    while stream.read(&mut buf).expect("failed unwrap stream value") > 0 {
        if buf.len() >= 1023 {
            return buf;
        }
    }

    buf
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

pub fn listen<F>(cbk: F) -> ()
where
    F: Fn([u8; 1024], SocketAddr) -> (),
{
    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed listen");
    let mut handlers: Vec<JoinHandle<(TcpStream, SocketAddr)>> = vec![
        accept(listener.try_clone().expect("failed clone listener")),
        accept(listener.try_clone().expect("failed clone listener")),
    ];
    let max = handlers.len();

    loop {
        let mut notyet: Vec<JoinHandle<(TcpStream, SocketAddr)>> = vec![];
        //
        while let Some(handler) = handlers.pop() {
            if handler.is_finished() {
                let (mut stream, address) = handler.join().expect("failed join handler");

                log::info!("Accepted: {:?}", address);

                let buf = handle_stream(&mut stream);
                stream
                    .shutdown(std::net::Shutdown::Both)
                    .expect("failed shutdown");

                log::info!("disconnected : {:?}", address);

                cbk(buf, address);
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
