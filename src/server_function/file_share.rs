use std::{fs, io};
use std::fs::File;
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
        let mut input = File::open(&self.path).unwrap();
        let mut result=String::new();
        input.read_to_string(&mut result).unwrap();
        stream.write(result.as_bytes());
    }

    pub fn file_download(&mut self) {
        let path= String::from(&self.path);
        //Determine whether the parent directory exists, if not exist, create the directory
        match path.rfind("/"){
            Some(v)=> {
                let dir= String::from(&path[0..v]);
                fs::create_dir_all(&dir).unwrap();
            },
            None=>()
        };
        let mut file = File::create(&self.path).unwrap();
        file.write(self.content.as_bytes()).unwrap();
    }

    pub fn create_dir(&mut self) {
        fs::create_dir_all(&self.path).unwrap();
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
        for entry in fs::read_dir(src).unwrap() {
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
    }
}

