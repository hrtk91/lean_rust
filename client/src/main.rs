use tcp::{Client, TcpClient};

mod command_args;
mod tcp;

fn main() {
    env_logger::init();
    let option = command_args::read_cmd_args();

    let client = Client::new(option);
    let stdin = std::io::stdin();
    loop {
        let mut buf = String::from("");
        stdin.read_line(&mut buf).expect("failed to readline");
        client.send(&buf).expect("failed send message");
    }
}

