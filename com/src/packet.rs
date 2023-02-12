use std::collections::VecDeque;

#[derive(Debug)]
pub struct Header {
    pub len: u32,
}

#[derive(Debug)]
pub struct Packet {
    pub header: Header,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn to_vec(&self) -> Vec<u8> {
        let len = self.header.len.to_be_bytes().to_vec();
        let payload = self.payload.clone();
        [len, payload].concat()
    }
}

enum State {
    Len,
    Payload,
    Completed,
}

pub fn parse(buf: Vec<u8>) -> Result<Packet, ()> {
    let mut buf = VecDeque::from(buf);
    let mut state = State::Len;
    let mut count = std::mem::size_of::<u32>();
    let mut len: u32 = 0;
    let mut payload: Vec<u8> = vec![];

    log::trace!("start parse data");

    log::trace!("{:?}", buf);

    let packet = loop {
        let byte = buf.pop_front();
        match state {
            State::Len => {
                log::trace!("enter len");
                let byte = match byte {
                    Some(byte) => byte,
                    None => return Err(()),
                };
                log::trace!("Len {}", byte);
                len = (len << 8) | (byte as u32);
                count = count - 1;
                if count == 0 {
                    log::trace!("Len to Payload {}", byte);
                    state = State::Payload
                }
            }
            State::Payload => {
                match byte {
                    Some(byte) => payload.push(byte),
                    None => state = State::Completed,
                };
            }
            State::Completed => {
                log::trace!("Read Payload completed");
                break Packet {
                    header: Header { len },
                    payload: payload.into(),
                };
            }
        };
    };

    Ok(packet)
}
