use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;



pub struct OpFile{
    name: String
}


impl  OpFile {
    pub fn new (name:String) -> OpFile{
        OpFile{name}
    }

    pub fn file_upload(&mut self, mut stream: &TcpStream){
        println!("Do you want to use the default path:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut input = input.replace("\n","");
        let path = match input.as_ref() {
            "yes"=>{
                String::from(&self.name)
            }
            "no"=>{
                println!("Input the path you want:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).expect("Failed to read line");
                path = path.replace("\n","");
                format!("{}/{}",path,self.name)
            }
            _=>{
                String::from("Error")
            }
        };
        println!("{}",path);
        let mut input = File::open(&self.name).unwrap();
        let together = format!("FileIn,{},",path);
        stream.write(together.as_bytes());
        let mut result=String::new();
        input.read_to_string(&mut result).unwrap();
        stream.write(result.as_bytes());
    }

    pub(crate) fn file_download(&mut self, mut stream: &TcpStream){
        //let path = format!("{}",")

        let together = format!("FileOut,{},Null",self.name);
        stream.write(together.as_bytes());

        let mut server_buf = [0;1024];
        let client_read = stream.read(&mut server_buf).unwrap();
        let mut content= std::str::from_utf8(&server_buf[..client_read]).unwrap();
        let mut file = File::create(&self.name).unwrap();
        file.write(content.as_bytes());
    }
}

