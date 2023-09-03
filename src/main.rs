use std::collections::HashMap;
use trust_tcp_ip::tcp;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: u32,
    dst: u32,
    sport: u16,
    dport: u16,
}

fn main() {
    let mut connections: HashMap<Quad, tcp::Connection> = Default::default();
    let mut nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).unwrap();
    println!("created tun device: {:?}", nic.name());
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..]).unwrap();
        let _flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        if proto != 0x800 {
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(header) => {
                let src = header.source_addr();
                let dst = header.destination_addr();
                let proto = header.protocol();

                if proto != 0x06 {
                    continue;
                }

                println!("ip protocol: {} > {}; protocol: {}",
                         src,
                         dst,
                         header.protocol());

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + header.slice().len()..nbytes]) {
                    Ok(tcp) => {
                        let datai = 4 + header.slice().len() + tcp.slice().len();
                        connections
                            .entry(Quad {
                                src: src.into(),
                                dst: dst.into(),
                                sport: tcp.source_port(),
                                dport: tcp.destination_port(),
                            })
                            .or_default()
                            .on_packet(
                                &mut nic,
                                header,
                                tcp,
                                &buf[datai..nbytes],
                            ).unwrap();
                    }
                    Err(e) => {
                        println!("忽略无效TCP包: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("忽略无效包: {:?}", e);
            }
        }
    }
}
