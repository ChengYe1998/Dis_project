use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

struct SoftDeploy{
    name: String,
    stream: TcpStream
}

impl SoftDeploy{
    fn new (name : String, stream : TcpStream) -> SoftDeploy {
        SoftDeploy{name, stream}
    }
    //This function is for transfer data to server.
    fn basic_soft(mut self, input: String){
        self.stream.write(input.as_bytes()).expect("failed to write");
        let mut server_buf = [0;1024];
        let client_read = self.stream.read(&mut server_buf).unwrap();
        let mut mess = std::str::from_utf8(&server_buf[..client_read-1]).unwrap();
        println!("read form server:{:?}", mess);
        println!();
    }

    pub fn install(mut self){
        let mut input = format!("install\n{}",self.name);
        soft_deploy::basic_soft(self, input);
    }

    pub fn update(mut self){
        let mut input = format!("update\n{}",self.name);
        soft_deploy::basic_soft(self, input);
    }

    pub fn remove(mut self){
        let mut input = format!("remove\n{}",self.name);
        soft_deploy::basic_soft(self, input);
    }

    pub fn query(mut self){
        let mut input = format!("query\n{}",self.name);
        soft_deploy::basic_soft(self, input);
    }
}
