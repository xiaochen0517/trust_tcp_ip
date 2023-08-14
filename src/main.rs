fn main() {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).unwrap();
    println!("created tun device: {:?}", nic.name());
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..]).unwrap();
        println!("read data : {:X?}", &buf[..nbytes]);
    }
}
