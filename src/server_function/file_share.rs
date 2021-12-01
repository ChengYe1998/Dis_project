use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;


pub struct OpFile{
    pub path: String,
    pub content:String
}

impl OpFile {
    pub fn new(path: String, content: String) -> OpFile {
        OpFile { path, content }
    }

    pub fn file_upload(&mut self,mut stream: &TcpStream){

        let mut input = File::open(&self.path).unwrap();
        let mut result=String::new();
        input.read_to_string(&mut result).unwrap();
        stream.write(result.as_bytes());
    }

    pub fn file_download(&mut self) {
        let pos :Vec<String> = self.path.split("/").map(String::from).collect();
        let mut final_path=String::from("/");
        for i in 0..pos.len()-1{
            final_path=format!("{}{}/",final_path,pos[i]);
        }
        let dir = format!("/root{}", final_path);
        fs::create_dir_all(&dir).unwrap();
        let together = format!("{}{}",dir,pos[pos.len()-1]);
        println!("{}",together);
        let mut file = File::create(together).unwrap();
        file.write(self.content.as_bytes()).unwrap();
        println!("succ");
    }
}