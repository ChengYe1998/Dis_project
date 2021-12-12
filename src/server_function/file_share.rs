use std::{fs, io};
use std::fs::{File, metadata};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;


pub struct OpFile{
    pub path: String,
    pub content:String
}

impl OpFile {
    pub fn new(path: String, content: String) -> Self{
        OpFile { path, content }
    }
    pub fn file_upload(&mut self,mut stream: &TcpStream){
        let mut result = match File::open(&self.path) {
            Ok(mut v)=>{
                let mut result=String::new();
                v.read_to_string(&mut result).unwrap();
                result
            }
            Err(_t)=>{
                let mut result=String::from("File opened error, please check this file exist or not");
                result
            }
        };
        let together = format!("NewFile,{},{}",self.path,result);
        stream.write(together.as_bytes());
        let mut buffer = [0;512];
        loop {
            let size = stream.read(&mut buffer).unwrap();
            let mess = std::str::from_utf8(&buffer[..size]).unwrap();
            //When received Ok, moved to next step
            if mess == "OK" {
                break;
            }
        }
    }
    pub fn file_download(&mut self) -> String{
        let path= String::from(&self.path);
        //Determine whether the parent directory exists, if not exist, create the directory
        match path.rfind("/"){
            Some(v)=> {
                let dir= String::from(&path[0..v]);
                fs::create_dir_all(&dir).unwrap();
            },
            None=>()
        };
        match metadata(&self.path){
            Ok(md)=>{
                if md.is_dir(){
                    String::from("Is a directory")
                }
                else {
                    fs::remove_file(&self.path).unwrap();
                    match File::create(&self.path) {
                        Ok(mut v)=>{
                            v.write(self.content.as_bytes()).unwrap();
                            String::from("OK")
                        },
                        Err(_t)=>{
                            String::from("Is a directory")
                        }
                    }
                }
            }
            Err(_e)=>{
                String::from("Is a directory")
            }
        }

    }
    pub fn create_dir(&mut self)->String {
        match fs::create_dir_all(&self.path){
            Ok(_v)=>String::from("OK"),
            Err(_t)=>String::from("File exists"),
        }
    }
    pub fn save_file(&mut self){
        let mut file = File::create(&self.path).unwrap();
        file.write(self.content.as_bytes()).unwrap();
    }
    pub fn directory_upload(&mut self, src: &str, dst: &str, mut stream: &TcpStream) {
        let together = format!("CreateDir,{},Null",dst);
        stream.write(together.as_bytes());
        stream.flush().unwrap();
        let mut buffer = [0;512];
        loop {
            let size = stream.read(&mut buffer).unwrap();
            let mess = std::str::from_utf8(&buffer[..size]).unwrap();
            //When received Ok, moved to next step
            if mess == "OK" {
                break;
            }
        }
        match fs::read_dir(src){
            Ok(T)=>{
                for entry in T {
                    let entry = entry.unwrap();
                    let ty = entry.file_type().unwrap();
                    stream.flush().unwrap();
                    if ty.is_dir() {
                        //recursion
                        let str=format!("{}/{}",dst,entry.file_name().to_str().unwrap());
                        self.directory_upload(entry.path().to_str().unwrap(), &str,stream);
                    } else {
                        //transfer file to client side
                        let mut input = File::open(&entry.path()).unwrap();
                        let str=format!("{}/{}",dst,entry.file_name().to_str().unwrap());
                        let mut result=String::new();
                        input.read_to_string(&mut result).unwrap();
                        let together = format!("SaveFile,{},{}",str,result);
                        stream.write(together.as_bytes());
                        loop{
                            let size= stream.read(&mut buffer).unwrap();
                            let mess=std::str::from_utf8(&buffer[..size]).unwrap();
                            if mess =="OK"{
                                break;
                            }
                        }
                    }
                }
            },
            Err(_E)=>{stream.write("Directory not exist".as_bytes()).unwrap();},
        };

    }
}

