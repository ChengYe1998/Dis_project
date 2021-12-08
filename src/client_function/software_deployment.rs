use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use crate::client_function::op_stream::ClientStream;

pub struct SoftDeploy{
    name: String,
}
impl SoftDeploy{
    pub fn new (name : String) -> Self {
        SoftDeploy{name}
    }
    pub fn install(&mut self) -> String{
        let mut input = format!("soft,install,{}",self.name);
        input
    }
    pub fn update(&mut self) -> String{
        let mut input = format!("soft,update,{}",self.name);
        input
    }
    pub fn remove(&mut self) -> String{
        let mut input = format!("soft,remove,{}",self.name);
        input
    }
    pub fn query(&mut self) -> String{
        let mut input = format!("soft,query,{}",self.name);
        input
    }
    pub fn install_by_version(&mut self,stream:&mut TcpStream){
        let mut input = format!("soft,version,{}",self.name);
        ClientStream::io_stream(stream, &input);
        let mut version = String::new();
        io::stdin().read_line(&mut version).expect("Failed to read line");
        ClientStream::io_stream(stream, &version);
    }
    //select which function should use
    pub fn select(&mut self, option: &String) -> String{
        match option.as_ref() {
            "install" => {
                self.install()
            },
            "update" => {
                self.update()
            },
            "remove" => {
                self.remove()
            },
            "query" => {
                self.query()
            },
            "install_by_version" => {
                String::from("install_by_version")
            },
            _ => {
                String::from("wrong option")
            }
        }
    }
}
