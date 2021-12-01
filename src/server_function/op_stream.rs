use std::io::{Read, Write};
use std::net::TcpStream;
use crate::Server;
use crate::server_function::file_share::OpFile;
use crate::server_function::server_monitor::MonServer;
use crate::server_function::software_deployment::SoftwareInfo;


pub struct ServerStream{
    pub stream :TcpStream,
    pub function :String
}

impl ServerStream {
    pub fn new()->ServerStream{
        let s = Server::new(String::from("7879"));
        let stream = s.connect_client();
        let function = String::from("");
        ServerStream{stream,function}
    }

    pub fn read_first(&mut self) -> Vec<String>{
        let mut client_buf = [0;512];
        let mut server_read = self.stream.read(&mut client_buf).unwrap();
        while server_read==0{
            server_read = self.stream.read(&mut client_buf).unwrap();
        }

        let mess = std::str::from_utf8(&client_buf[..server_read]).unwrap();
        println!("{}",mess);
        let pos :Vec<String> = mess.split(",").take(3).map(String::from).collect();
        println!("{:?}",pos);
        pos
    }

    /*pub fn read_first(&mut self) -> Vec<String>{
        println!("1");
        let mut together=String::from("");

        let mut received: Vec<u8> = vec![];
        let mut client_buf= [0; 512];
        self.stream.read(&mut client_buf);

        // Array with a fixed size
        let mut rx_bytes = [0u8; 512];
        loop {
            // Read from the current data in the TcpStream
            let bytes_read = self.stream.read(&mut rx_bytes).unwrap();

            // However many bytes we read, extend the `received` string bytes
            received.extend_from_slice(&rx_bytes[..bytes_read]);

            // If we didn't fill the array
            // stop reading because there's no more data (we hope!)
            if bytes_read < 512 {
                break;
            }
        }




        /*loop {
            println!("2");
            let mut client_buf= [0; 512];
            match self.stream.read(&mut client_buf) {
                Ok(n) => {
                    if n == 0 {
                        println!("break");
                        break;
                    } else {
                        let mut mess = std::str::from_utf8(&client_buf[..n]).unwrap();
                        together= together.add(mess);
                        print!("{}",together);
                    }
                }
                Err(e) =>{
                    println!("Error in reading stream data: {:?}", e);
                    break;
                }
            }
        }*/
        println!("3");
        together= std::str::from_utf8(&received[..]).unwrap().parse().unwrap();
        let pos :Vec<String> = together.split(",").take(3).map(String::from).collect();
        pos
    }*/

    pub fn read_client(&mut self) -> String{
        let mut client_buf = [0; 512];
        let server_read = self.stream.read(&mut client_buf).unwrap();
        let mess = std::str::from_utf8(&client_buf[..server_read]).unwrap();
        //let pos: Vec<&str> = mess.split(",").collect();
        String::from(mess)
    }
    pub fn soft_selected(&mut self, method:String, name:String){
        let mut soft = SoftwareInfo::new(name, String::from(""));
        let result = soft.selected(method,self);
        self.stream.write(result.as_bytes()).unwrap();
    }
    pub fn mon_selected(&mut self, mon_type:String, option:String){
        let mut mon_server=MonServer::new(mon_type,option);
        let result = mon_server.select();
        self.stream.write(result.as_bytes()).unwrap();
    }
    pub fn file_download(&mut self, path:String, content:String){
        let mut op_file=OpFile{ path,content };
        op_file.file_download();
        //self.stream.write(result.as_bytes()).unwrap();
    }
    pub fn file_upload(&mut self, path:String, content:String){
        let mut op_file=OpFile{ path,content };
        op_file.file_upload(&self.stream);
    }
    pub fn select(&mut self){
        let pos = self.read_first();
        let data_one = pos[1].clone();
        let data_two = pos[2].clone();
        match pos[0].as_ref(){
            "soft"=>self.soft_selected(data_one,data_two),
            "mon"=>self.mon_selected(data_one,data_two),
            "FileIn"=>self.file_download(data_one,data_two),
            "FileOut"=>self.file_upload(data_one,data_two),
            _ => {}
        }
    }
}