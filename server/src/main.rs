use std::net::SocketAddr;
mod tcp;

fn main() {
    env_logger::init();
    log::info!("listen started at 127.0.0.1:8080");
    tcp::listen(handle_receive);
}

fn handle_receive(value: [u8; 1024], address: SocketAddr) {
    let mut len = 0;
    while value[len] != 0 {
        len = len + 1
    }
    if let Ok(str) = std::str::from_utf8(&value[0..len]) {
        log::info!("{:?} : {:?}", address, str);
    }
}
