fn main() {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).unwrap();
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
                        let src_port = tcp.source_port();
                        let dst_port = tcp.destination_port();
                        println!("tcp protocol: {}:{} > {}:{}; {} bytes",
                                 src,
                                 src_port,
                                 dst,
                                 dst_port,
                                 tcp.slice().len());
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
