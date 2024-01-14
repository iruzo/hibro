use std::net::{TcpListener, TcpStream};


fn handle(stream: TcpStream){

    println!("{:?}", stream) ;

}

pub fn open(url: &str, port: &str) -> std::io::Result<()> {

    let mut localhost = String::from(url);
    localhost.push_str(port);
    let listener = TcpListener::bind(localhost)?;

    for stream in listener.incoming() {
        handle(stream?);
    }
    Ok(())
}
