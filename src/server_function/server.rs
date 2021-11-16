
use std ::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::time;
use std::io;
use std::io::{Read,Write};
use std::process::Command;
use crate::server_function::software::{install, installed, remove, update};
//Interact with the server
pub fn handle_client(mut stream: TcpStream) -> io::Result<()>{

    let mut client_buf = [0;512];
    let mut done = false;

    while !done{
        let server_read = stream.read(&mut client_buf)?;
        let mut mess = std::str::from_utf8(&client_buf[..server_read-1]).unwrap().lines();
        let option = match mess.next() {
            None => "Error",
            Some(ref x) => x // x is now a string slice
        };
        let name = match mess.next() {
            None => "Error",
            Some(ref x) => x // x is now a string slice
        };
        let mut result = match option {
            "1" => install(name),
            "2" => update(name),
            "3" => remove(name),
            "4" => String::from(installed(name)),
            _ => String::from("Invalid input")
        };
        stream.write(result.as_bytes())?;
    }
    Ok(())
}