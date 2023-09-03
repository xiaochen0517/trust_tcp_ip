use std::io;
use tun_tap::Iface;

pub enum State {
    Closed,
    Listen,
}

pub struct Connection {
    pub state: State,
}

pub struct SendSequenceSpace {
    pub unacknowledged: u32,
    pub next: u32,
    pub window: u16,
}

pub struct ReceiveSequenceSpace {
    pub next: u32,
    pub window: u16,
}

impl Default for Connection {
    fn default() -> Self {
        // State::Closed
        Connection {
            state: State::Listen,
        }
    }
}

impl Connection {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<u8> {
        let mut buf = [0u8; 1504];
        match (*self).state {
            State::Closed => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Connection is closed",
                ));
            }
            State::Listen => {
                if !tcph.syn() {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Connection is not established",
                    ));
                }

                let mut syn_ack = etherparse::TcpHeader::new(
                    tcph.destination_port(),
                    tcph.source_port(),
                    unimplemented!(),
                    unimplemented!(),
                );
                syn_ack.syn = true;
                syn_ack.ack = true;

                let mut ip_packet = etherparse::Ipv4Header::new(
                    syn_ack.header_len(),
                    64,
                    etherparse::IpNumber::Tcp as u8,
                    iph.destination_addr().octets(),
                    iph.source_addr().octets(),
                );
                let unwriten_size = {
                    let mut unwriten = &mut buf[..];
                    ip_packet.write(&mut unwriten);
                    syn_ack.write(&mut unwriten);
                    unwriten.len()
                };
                nic.send(&buf[..unwriten_size])?;
            }
        }
        eprintln!(
            "{}:{} > {}:{}; {} bytes",
            iph.source_addr(),
            tcph.source_port(),
            iph.destination_addr(),
            tcph.destination_port(),
            data.len()
        );
        Ok(0)
    }
}