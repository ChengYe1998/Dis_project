use std::fs::{File, remove_file};
use std::{fs, io};
use std::io::{BufRead, BufReader, Read, Write};
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
    pub fn new(vec_stream: Vec<TcpStream>, function: String) ->Self{
            ClientStream{vec_stream,function}
    }
    //for the information transmission
    pub fn io_stream(mut stream: &mut TcpStream, input: &String) -> String{
        stream.write(input.as_bytes()).expect("failed to write");
        let mut server_buf = [0;512];
        let mut client_read;
        let mut longfile: Vec<u8> = vec![];
        loop {
            server_buf = [0;512];
            client_read = stream.read(&mut server_buf).unwrap();
            longfile.extend_from_slice(&server_buf[..client_read]);
            if client_read<512{
                break;
            }
        }
        let mess = std::str::from_utf8(&longfile).unwrap();
        let mess = String::from(mess);
        for line in mess.lines(){
            println!("{}", line);
        }
        println!();
        mess
    }
    //for file transmission
    pub fn io_file(mut stream: &TcpStream) -> Vec<String>{
        let mut server_buf = [0;512];
        let mut client_read;
        let mut longfile: Vec<u8> = vec![];
        loop {
            server_buf = [0;512];
            client_read = stream.read(&mut server_buf).unwrap();
            longfile.extend_from_slice(&server_buf[..client_read]);
            if client_read<512{
                break;
            }
        }
        let mess = std::str::from_utf8(&longfile).unwrap();

        let mut pos:Vec<String> = mess.split(",").take(2).map(String::from).collect();
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
        pos
    }
    //for choose single server node in a group
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
        println!("Choose what you want to do(install, query, update, install_by_version, remove):");
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
            if result=="install_by_version"{
                soft_deploy.install_by_version(stream);
            }
            else{
                ClientStream::io_stream(stream,&result);
            }
        }
    }
    pub fn mon_selected(&mut self){
        println!("Input which type you want to monitor(cpu, memory, system, network or user):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut input = input.replace("\n","");
        match input.as_ref(){
            "cpu"=> println!("Choose logical, physical or detail:"),
            "memory"=>println!("Choose usage, free or total:"),
            "network"=>println!("Choose interface, firewall, routing, connection, lis_ports or net_info:"),
            "system"=>println!("Choose  sys_info, kernel_module, environment:"),
            _=>println!("Choose active, log, groups, users:"),
        }
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
        println!("Choose what you want to do(upload file, download file, upload directory, download directory, compare directory):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut input = input.replace("\n","");
        println!("Input the file or directory path:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let mut name = name.replace("\n","");
        let mut name_copy = String::from(&name);
        let mut file = OpFile::new(name_copy);
        match input.as_ref(){
            "upload file"=>{
                println!("Do you want to use the default path?");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let mut input = input.replace("\n","");
                let path = match input.as_ref() {
                    "yes"=>{
                        let path = match name.rfind("/"){
                            Some(v)=>{
                                let file= String::from(&name[v+1..]);
                                file
                            }
                            None=>{
                                String::from(&name)
                            }
                        };
                        path
                    }
                    _=>{
                        println!("Input the path you want:");
                        let mut path = String::new();
                        io::stdin().read_line(&mut path).expect("Failed to read line");
                        path = path.replace("\n","");
                        path
                    }
                };
                println!("Do you want to upload to all servers:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let mut input = input.replace("\n","");
                match input.as_ref(){
                    "yes"=>{
                        for stream in &mut self.vec_stream{
                            file.file_upload(&path,stream);
                        }
                    },
                    _=>{
                        let stream = self.choose_server_node();
                        file.file_upload(&path, stream);
                    }
                }
            }
            "download file"=>{
                let stream = self.choose_server_node();
                println!("Do you want to use the default path to save file?");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let mut input = input.replace("\n","");
                let file_path = match name.rfind("/"){
                    Some(v)=>{
                        let file= String::from(&name[v+1..]);
                        file
                    }
                    None=>{
                        String::from(&name)
                    }
                };
                let path = match input.as_ref(){
                    "yes"=>{file_path}
                    _=>{
                        println!("Input the the path you want to save");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        let mut input = input.replace("\n","");
                        input
                    }
                };
                file.file_download(&path,stream);
            }

            "upload directory"=>{
                println!("Input the destination path:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).expect("Failed to read line");
                let mut path = path.replace("\n","");

                println!("Do you want to upload to all servers:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let mut input = input.replace("\n","");
                match input.as_ref(){
                    "yes"=>{
                        for stream in &mut self.vec_stream{
                            file.directory_upload(&name,&path,& stream);
                        }
                    },
                    _=>{
                        let mut stream = self.choose_server_node();
                        file.directory_upload(&name,&path,& stream);
                    }
                }
            }
            "download directory"=>{
                let stream = self.choose_server_node();
                println!("Do you want to use the default path to save directory?");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let mut input = input.replace("\n","");
                let file_path = match name.rfind("/"){
                    Some(v)=>{
                        let file= String::from(&name[v+1..]);
                        file
                    }
                    None=>{
                        String::from(&name)
                    }
                };
                let path = match input.as_ref(){
                    "yes"=>{file_path}
                    _=>{
                        println!("Input the the path you want to save");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        let mut input = input.replace("\n","");
                        input
                    }
                };
                file.directory_download(&path,stream);
            }
            "compare directory"=>{
                let stream = self.choose_server_node();
                let name_clone = file.directory_compare(stream);
                let mut diff_file = File::open("compare_file").unwrap();
                let buffer = BufReader::new(diff_file);
                for line in buffer.lines(){
                    println!("{}",line.unwrap());
                }
                fs::remove_file("compare_file").unwrap();
                println!();
                println!("Do you want to update your local file or remote file(any other input will do nothing)?");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let mut input = input.replace("\n","");
                match input.as_ref(){
                    "local file"=>{
                        match fs::remove_dir_all(&name){
                            Ok(T)=>{},
                            Err(E)=>{}
                        }
                        fs::rename(name_clone, name).unwrap();
                    }
                    "remote file"=>{
                        file.directory_upload(&name,&name,& stream);
                    }
                    _=>{
                        fs::remove_dir_all(&name_clone).unwrap();
                        match fs::remove_dir_all(&name_clone){
                            Ok(T)=>{},
                            Err(E)=>{}
                        }
                    }
                }

            }
            _ => {}
        }
    }
    //for choose which function user want to use
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