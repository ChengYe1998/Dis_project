use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;
use proc_macro::tracked_path::path;

struct FileInfo {
    name: String,
    size: usize
}

struct OpFile{
    path: String,
    stream: TcpStream
}

impl  OpFile {
    fn new (path:String, stream: TcpStream) -> OpFile{
        Op_File{path, stream}
    }
    fn file_upload(&mut self){
        let input = File::open(&self.path)?;
        let pos = self.path.rfind("/").unwrap();
        let name = &self.path[pos+1..]+"\n";

        self.stream.write(name.as_bytes());
        let buffered = BufReader::new(input);

        for line in buffered.lines() {
            self.stream.write(line.unwrap().as_bytes());
        }


    }
    fn file_download(&mut self){

        let mut file = File::create("/tmp/dst").unwrap();

        loop {
            let mut server_buf= [0; 512];
            match self.stream.read(&mut server_buf) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    } else {
                        let mut mess = std::str::from_utf8(&server_buf[..client_read-1]).unwrap();
                        file.write(mess.as_bytes()).unwrap();
                    }
                }
                Err(e) =>{
                    println!("Error in reading stream data: {:?}", e);
                    break;
                }

            }
        }
    }
}