use std::fs::metadata;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use crate::server_function::file_share::OpFile;
use crate::server_function::server_monitor::MonServer;
use crate::server_function::software_deployment::SoftwareInfo;

pub struct ServerStream{
    pub stream :TcpStream,
    pub function :String
}

impl ServerStream {
    pub fn new()->Self{
        let together = format!("0.0.0.0:7879");
        let listener = TcpListener::bind(together).expect("bind failed");
        let stream = listener.incoming().next().unwrap().unwrap();
        let function = String::from("");
        ServerStream{stream,function}
    }

    //use this function to read each request.
    pub fn read_first(&mut self) -> Vec<String>{
        let mut client_buf = [0;512];
        let mut server_read;
        let mut longfile: Vec<u8> = vec![];
        loop {
            client_buf = [0;512];
            server_read = self.stream.read(&mut client_buf).unwrap();
            longfile.extend_from_slice(&client_buf[..server_read]);
            if server_read<512&&server_read!=0{
                break;
            }
        }
        let mess = std::str::from_utf8(&longfile).unwrap();
        let mut pos:Vec<String> = mess.split(",").take(2).map(String::from).collect();
        if pos.len()>=2{
            let mut len=pos[0].len()+pos[1].len();
            let mut str;
            if mess.len()>len{
                len+=2;
                str = String::from(&mess[len..]);
            }
            else {
                str=String::new();
            }
            pos.push(str);
        }
        pos
    }

    //use this function to read sub-request.
    pub fn read_client(&mut self) -> String{
        let mut client_buf = [0;512];
        let mut server_read;
        let mut longfile: Vec<u8> = vec![];
        loop {
            client_buf = [0;512];
            server_read = self.stream.read(&mut client_buf).unwrap();
            longfile.extend_from_slice(&client_buf[..server_read]);
            if server_read<512{
                break;
            }
        }
        let mess = std::str::from_utf8(&longfile).unwrap();
        String::from(mess)
    }

    //if user want to use software related function, call this
    pub fn soft_selected(&mut self, method:String, name:String){
        let mut soft = SoftwareInfo::new(name, String::from(""));
        let result = soft.selected(method,self);
        self.stream.write(result.as_bytes()).unwrap();
    }

    //if user want to use monitoring related function, call this
    pub fn mon_selected(&mut self, mon_type:String, option:String){
        let mut mon_server=MonServer::new(mon_type,option);
        let result = mon_server.select();
        self.stream.write(result.as_bytes()).unwrap();
    }

    pub fn file_download(&mut self, path:String, content:String){
        let mut op_file=OpFile{ path,content };
        let result = op_file.file_download();
        self.stream.write(result.as_bytes());
    }

    pub fn file_upload(&mut self, path:String, content:String){
        let mut op_file=OpFile{ path,content };
        op_file.file_upload(&self.stream);
        self.stream.write("Finished,Null,Null".as_bytes());
    }

    pub fn directory_upload(&mut self, path:String, content:String){
        let path_1=String::from(&path);
        let path_2=String::from(&content);
        let mut op_file=OpFile{ path,content };
        op_file.directory_upload(&path_1, &path_2, &self.stream);
        self.stream.write("Finished,Null,Null".as_bytes());
    }

    pub fn create_dir(&mut self, path:String, content:String){
        let mut op_file=OpFile{ path,content };
        let result= op_file.create_dir();
        self.stream.write(result.as_bytes());
    }

    pub fn save_file(&mut self, path:String, content:String){
        let mut op_file=OpFile{ path,content };
        op_file.save_file();
        self.stream.write("OK".as_bytes());
    }

    //Process the request from client
    pub fn select(&mut self) {
        loop{
            let pos = self.read_first();
            if pos.len()>=3{
                let data_one = pos[1].clone();
                let data_two = pos[2].clone();
                match pos[0].as_ref(){
                    "soft"=>self.soft_selected(data_one,data_two),
                    "mon"=>self.mon_selected(data_one,data_two),
                    "FileIn"=>self.file_download(data_one,data_two),
                    "FileOut"=>{
                        let md = metadata(&data_one).unwrap();
                        if md.is_dir(){
                            self.directory_upload(data_one,data_two)
                        }
                        else if md.is_file(){
                            self.file_upload(data_one,data_two)
                        }
                    },
                    "CreateDir"=>self.create_dir(data_one,data_two),
                    "SaveFile"=>self.save_file(data_one,data_two),
                    _ => {}
                }
            }
            else if pos.len()!=0&&pos[0]=="exit"{
                break;
            }
        }
    }
}


