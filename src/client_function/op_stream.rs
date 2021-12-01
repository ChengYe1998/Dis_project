use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use serde_json::from_str;
use crate::client_function::file_share::{OpFile};
use crate::client_function::server_monitor::MonServer;
use crate::client_function::software_deployment::SoftDeploy;

pub struct ClientStream{
    pub vec_stream :Vec<TcpStream>,
    pub function :String
}

impl ClientStream {
    pub fn new(vec_stream: Vec<TcpStream>, function: String) ->ClientStream{
        ClientStream{vec_stream,function}
    }
    pub fn io_stream(stream: &mut TcpStream, input: &String){
        stream.write(input.as_bytes()).expect("failed to write");
        let mut server_buf = [0;1024];
        let client_read = stream.read(&mut server_buf).unwrap();
        let mut mess= std::str::from_utf8(&server_buf[..client_read]).unwrap();
        println!("read from server:");
        for line in mess.lines(){
            println!("{}", line);
        }
        println!();
    }

    pub fn choose_server_node(&mut self) -> &TcpStream {
        let len = self.vec_stream.len();
        println!("Now,we have {} nodes,choose the server node you want:",len);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut input = input.replace("\n","");
        let pos = input.parse::<usize>().unwrap();
        let stream = &self.vec_stream[pos];
        stream
    }

    pub fn soft_selected(&mut self)  {
        println!("Input the software name:");
        let mut soft_name = String::new();
        io::stdin().read_line(&mut soft_name).expect("Failed to read line");
        let mut soft_name = soft_name.replace("\n","");
        let mut soft_deploy = SoftDeploy::new(soft_name);
        println!("Choose what you want to do:");
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        let mut option = option.replace("\n", "");
        let mut result= soft_deploy.select(&option);
        while result=="wrong option"{
            println!("Please choose the right option:");
            option = String::from("");
            io::stdin().read_line(&mut option).expect("Failed to read line");
            option = option.replace("\n", "");
            result = soft_deploy.select(&option);
        }
        for stream in &mut self.vec_stream {
            ClientStream::io_stream(stream,&result);
        }
    }

    pub fn mon_selected(&mut self){
        println!("Input which type you want to monitor:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut input = input.replace("\n","");

        println!("Choose option:");
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        let mut option = option.replace("\n","");
        let mut mon_server = MonServer::new(input, option);
        let mut result = mon_server.select();
        while result=="Wrong option"{
            println!("Please choose the right option:");
            option = String::new();
            io::stdin().read_line(&mut option).expect("Failed to read line");
            option = option.replace("\n","");
            mon_server.option=option;
            result = mon_server.select();
        }
        for stream in &mut self.vec_stream{
            ClientStream::io_stream(stream, &result);
        }
    }

    pub fn file_selected(&mut self){
        println!("Choose what you want to do:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut input = input.replace("\n","");
        println!("Input the path of the file:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let mut name = name.replace("\n","");
        let mut file = OpFile::new(name);
        match input.as_ref(){
            "upload"=>{
                let stream = self.choose_server_node();
                file.file_upload(stream);
            }
            "download"=>{
                let stream = self.choose_server_node();
                file.file_download(stream);
            }
            _ => {}
        }
    }

    pub fn select_function(&mut self) -> bool{
        match self.function.as_ref(){
            "software"=>{
                self.soft_selected();
                true
            },
            "monitor"=>{
                self.mon_selected();
                true
            }
            "file"=>{
                self.file_selected();
                true
            }
            _ => false
        }
    }
}