use std::{fs, io};
use std::fs::{create_dir, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use crate::client_function::op_stream::ClientStream;

pub struct OpFile{
    name: String
}
impl  OpFile {
    pub fn new (name:String) -> Self{
        OpFile{name}
    }
    pub fn file_upload(&mut self, remote_path: &str,mut stream: &TcpStream){
        let mut input = File::open(&self.name).unwrap();
        let mut result=String::new();
        input.read_to_string(&mut result).unwrap();
        let together = format!("FileIn,{},{}",remote_path,result);
        stream.write(together.as_bytes());
    }
    pub fn file_download(&mut self, path:&str, mut stream: &TcpStream){
        let together = format!("FileOut,{},Null",self.name);
        stream.write(together.as_bytes());
        let mut server_buf = [0;512];
        let client_read = stream.read(&mut server_buf).unwrap();
        let mut content= std::str::from_utf8(&server_buf[..client_read]).unwrap();
        let mut file = File::create(path).unwrap();
        file.write(content.as_bytes());
    }
    pub fn directory_upload(&mut self, src: &str, dst: &str, mut stream: &TcpStream) {
        //tell the server the request type
        let together = format!("CreateDir,{},Null",dst);
        stream.write(together.as_bytes());
        stream.flush().unwrap();
        let mut buffer = [0;512];
        loop{
            let size= stream.read(&mut buffer).unwrap();
            let mess=std::str::from_utf8(&buffer[..size]).unwrap();
            if mess =="OK"{
                break;
            }
        }
        for entry in fs::read_dir(src).unwrap() {
            let entry = entry.unwrap();
            let file_type = entry.file_type().unwrap();
            stream.flush().unwrap();
            if file_type.is_dir() {
                //recursion
                let str=format!("{}/{}",dst,entry.file_name().to_str().unwrap());
                self.directory_upload(entry.path().to_str().unwrap(), &str,stream);
            }
            else {
                //transfer file
                let mut input = File::open(&entry.path()).unwrap();
                let str=format!("{}/{}",dst,entry.file_name().to_str().unwrap());
                let mut result=String::new();
                input.read_to_string(&mut result).unwrap();
                // stream.write(result.as_bytes());
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
    pub fn directory_download(&mut self, path: &str, mut stream: &TcpStream){
        let together = format!("DownloadDir,{},{}",self.name,path);
        stream.write(together.as_bytes());
        //Process the data from server side
        loop{
            let pos = ClientStream::io_file(stream);
            let data_one = pos[1].clone();
            let data_two = pos[2].clone();
            match pos[0].as_ref(){
                "CreateDir"=>{
                    fs::create_dir_all(data_one).unwrap();
                    stream.write("OK".as_bytes());
                },
                "SaveFile"=>{
                    let mut file = File::create(data_one).unwrap();
                    file.write(data_two.as_bytes()).unwrap();
                    stream.write("OK".as_bytes());
                },
                "Finished"=>{
                    break;
                }
                _ => {}
            }
        }
    }
    pub fn directory_compare(&mut self, mut stream: &TcpStream) -> String{
        //download whole directory to the local,
        //compare each file, if the file is not exist or different, write in to compare_file.
        let together = format!("DownloadDir,{},Null",self.name);
        let mut local_dir = format!("{}",self.name);
        self.name = format!("{}_clone",self.name);
        stream.write(together.as_bytes());
        let mut compare_file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("compare_file")
            .unwrap();
        loop{
            let pos = ClientStream::io_file(stream);
            let mut data_one=pos[1].clone();
            let data_two = pos[2].clone();
            match pos[0].as_ref(){
                "CreateDir"=>{
                    let mut pos:Vec<String> = data_one.split("/").take(1).map(String::from).collect();
                    let mut len=pos[0].len();
                    let mut str;
                    if data_one.len()>len{
                        len+=1;
                        str = String::from(&data_one[len..]);
                    }
                    else {
                        str=String::new();
                    }
                    pos.push(str);
                    if pos.len()==1{
                        data_one=format!("{}",self.name);
                    }
                    else{
                        data_one=format!("{}/{}",self.name,pos[1]);
                    }
                    fs::create_dir_all(data_one).unwrap();
                    stream.write("OK".as_bytes());
                },
                "SaveFile"=>{
                    let mut pos:Vec<String> = data_one.split("/").take(1).map(String::from).collect();
                    let mut len=pos[0].len();
                    let mut str;
                    if data_one.len()>len{
                        len+=1;
                        str = String::from(&data_one[len..]);
                    }
                    else {
                        str=String::new();
                    }
                    pos.push(str);
                    let local_file=format!("{}/{}",local_dir,pos[1]);
                    let remote_file=format!("{}/{}",self.name,pos[1]);
                    let mut file = File::create(&remote_file).unwrap();
                    file.write(data_two.as_bytes()).unwrap();
                    file.flush();
                    match File::open(&local_file){
                        Ok(t)=>{
                            let diff_title=format!("file {}: \n",&local_file);
                            compare_file.write(diff_title.as_bytes());
                            compare_file.flush();
                            self.file_compare(local_file, remote_file);
                        }
                        Err(e)=>{
                            let content = format!("{} is not exist in local\n",&local_file);
                            compare_file.write(content.as_bytes());
                            compare_file.flush();
                        }
                    }
                    stream.write("OK".as_bytes());
                },
                "Finished"=>{
                    break;
                }
                _ => {}
            }
        }
        String::from(&self.name)
    }
    pub fn file_compare(&mut self, local_name:String, remote_name:String){
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("compare_file")
            .unwrap();
        let mut local_file = File::open(&local_name).unwrap();
        let mut remote_file = File::open(&remote_name).unwrap();
        let mut local_len = BufReader::new(&local_file).lines().count();
        let mut remote_len = BufReader::new(&remote_file).lines().count();
        let mut local_file = File::open(&local_name).unwrap();
        let mut remote_file = File::open(&remote_name).unwrap();
        let mut local_buffer = BufReader::new(&local_file);
        let mut remote_buffer = BufReader::new(&remote_file);
        let mut count = 0;
        //compare two files each line
        while count!=local_len&&count!=remote_len {
            let mut line1= String::new();
            local_buffer.read_line(&mut line1);
            let mut line2= String::new();
            remote_buffer.read_line(&mut line2);
            count+=1;
            if line1!=line2{
                let mut content = format!("line {} is different: {}",count, line2 );
                file.write(content.as_bytes());
                file.flush();
            }
        }
    }
}


