use std::net::UdpSocket;
use clap::Error;

pub fn open (url: &str, port: &str) -> Result<(), Error>{

    let mut localhost = String::from(url);
    let port = port;
    localhost.push_str(":");
    localhost.push_str(&port);

    let socket = UdpSocket::bind(localhost)?;

    let mut buf = [0; 64];
    let (amt, src) = socket.recv_from(&mut buf)?;

    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, &src)?;
    println!("{:?}", socket);

    Ok(())

}
