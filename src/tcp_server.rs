use std::io::{Read, Write};
use std::new::{TcpListener, TcpStream};
use std::{str, thread};

pub fn serve(address: &str) -> Result<(), String> {
    let listener = TcpListener::bind(address)?;
    loop {
        let (stream, _) = listener.accept()?;
        thread::spawn(move || {
            handoler(stream).unwrap_or_else(|err| error!("{:?}", err));
        })
    }
}

fn handler(mut stream: TcpStream) -> Result<(), String> {
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(());
        }
        print!("{}", str::from_utf8(&buffer[..nbytes])?);
        stream.write_all(&buffer[..nbytes])?;
    }
}