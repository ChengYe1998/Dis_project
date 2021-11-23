use std::io::{self,prelude::*,BufReader,Write};
use std::net::TcpStream;
use std::str;


pub fn input(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    println!("Input the name:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    stream.write(input.as_bytes()).expect("failed to write");
}

pub fn handle_server(mut stream: TcpStream) -> io::Result<()> {

    let mut done = false;
    while !done {

        let mut server_buf = [0;1024];
        let client_read = stream.read(&mut server_buf)?;
        let mut mess = std::str::from_utf8(&server_buf[..client_read-1]).unwrap();
        println!("read form server:{:?}", mess);
        println!();

    }
    Ok(())
}