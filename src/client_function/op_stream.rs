use std::fs::{File, metadata, remove_file};
use std::{fs, io};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpStream};
use serde_json::from_str;
use crate::client_function::file_share::{OpFile};
use crate::client_function::server_monitor::MonServer;
use crate::client_function::software_deployment::SoftDeploy;
extern crate serde;
use serde::{Deserialize, Serialize};


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
            if client_read<512&&client_read!=0{
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
            if client_read<512&&client_read!=0{
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
        input = input.trim().parse().unwrap();//replace("\n","");
        let pos = input.parse::<usize>().unwrap();
        let stream = &self.vec_stream[pos];
        stream
    }
    pub fn soft_selected(&mut self)  {
        println!("Input the software name:");
        let mut soft_name = String::new();
        io::stdin().read_line(&mut soft_name).expect("Failed to read line");
        soft_name = soft_name.trim().parse().unwrap();
        let mut soft_deploy = SoftDeploy::new(soft_name);
        println!("Choose what you want to do(install, query, update, install_by_version, remove):");
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        option = option.trim().parse().unwrap();
        let mut result= soft_deploy.select(&option);
        while result=="wrong option"{
            println!("Please choose the right option:");
            option = String::from("");
            io::stdin().read_line(&mut option).expect("Failed to read line");
            option = option.trim().parse().unwrap();
            result = soft_deploy.select(&option);
        }
        for stream in &mut self.vec_stream {
            let mut io_result=String::new();
            if result=="install_by_version"{
                io_result= soft_deploy.install_by_version(stream);
            }
            else{
                io_result = ClientStream::io_stream(stream,&result);
            }
            while io_result=="Software not exists"{
                println!("Input the right software name:");
                let mut soft_name = String::new();
                io::stdin().read_line(&mut soft_name).expect("Failed to read line");
                soft_name = soft_name.trim().parse().unwrap();
                soft_deploy.name=soft_name;
                result = soft_deploy.select(&option);
                if result=="install_by_version"{
                    io_result= soft_deploy.install_by_version(stream);
                }
                else{
                    io_result = ClientStream::io_stream(stream,&result);
                }
            }
        }
    }
    pub fn mon_selected(&mut self){
        println!("Input which type you want to monitor(cpu, memory, system, network or user):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().parse().unwrap();
        match input.as_ref(){
            "cpu"=> println!("Choose logical, physical or detail:"),
            "memory"=>println!("Choose usage, free or total:"),
            "network"=>println!("Choose interface, firewall, routing, connection, lis_ports or net_info:"),
            "system"=>println!("Choose  sys_info, kernel_module, environment:"),
            _=>println!("Choose active, log, groups, users:"),
        }
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        option = option.trim().parse().unwrap();
        let mut mon_server = MonServer::new(input, option);
        let mut result = mon_server.select();
        while result=="Wrong option"{
            println!("Please choose the right option:");
            option = String::new();
            io::stdin().read_line(&mut option).expect("Failed to read line");
            option = option.trim().parse().unwrap();
            mon_server.option=option;
            result = mon_server.select();
        }
        for stream in &mut self.vec_stream{
            ClientStream::io_stream(stream, &result);
        }
    }
    pub fn file_selected(&mut self){
        println!("Choose what you want to do(upload file, download file, compare file):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().parse().unwrap();
        loop {
            match input.as_ref() {
                "upload file"=>break,
                "download file"=>break,
                "compare file"=>break,
                _=>{
                    println!("Please select the right option:");
                    input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    input = input.trim().parse().unwrap();
                }
            }
        }

        println!("Input the file or directory path:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        name = name.trim().parse().unwrap();
        let mut name_copy = String::from(&name);
        let mut file = OpFile::new(name_copy);
        match input.as_ref(){
            "upload file"=>{
                println!("Do you want to use the default path to save file?");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                input = input.trim().parse().unwrap();
                let remote_path = match input.as_ref() {
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
                        path = path.trim().parse().unwrap();
                        path
                    }
                };
                println!("Do you want to upload to all servers:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                input = input.trim().parse().unwrap();
                match input.as_ref() {
                    "yes" => {
                        match metadata(&name) {
                            Ok(md) => {
                                if md.is_dir() {
                                    for stream in &mut self.vec_stream {
                                        file.directory_upload(&name, &remote_path, &stream);
                                    }
                                } else if md.is_file() {
                                    for stream in &mut self.vec_stream {
                                        file.file_upload(&remote_path, stream);
                                    }
                                }
                            }
                            Err(_t) => {
                                println!("No such file or directory");
                            }
                        };
                    },
                    _ => {
                        let stream = self.choose_server_node();
                        match metadata(&name) {
                            Ok(md) => {
                                if md.is_dir() {
                                    file.directory_upload(&name, &remote_path, &stream);
                                } else if md.is_file() {
                                    file.file_upload(&remote_path, stream);
                                }
                            }
                            Err(_t) => {
                                println!("No such file or directory");
                            }
                        }
                    }
                }
            }
            "download file"=>{
                let stream = self.choose_server_node();
                println!("Do you want to use the default path to save file?");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                input = input.trim().parse().unwrap();
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
                        input = input.trim().parse().unwrap();
                        input
                    }
                };
                match metadata(&path) {
                    Ok(_md) => {
                        file.file_download(&path,stream);
                    }
                    Err(_t) => {
                        println!("No such file or directory");
                    }
                }
            }
            "compare file"=>{
                let stream = self.choose_server_node();
                println!("The file is in the default path?");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                input = input.trim().parse().unwrap();
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
                        println!("Input the the path of the local file");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        input = input.trim().parse().unwrap();
                        input
                    }
                };
                let name_clone= match metadata(&path) {
                    Ok(_md) => {
                        file.directory_compare(&path,stream)
                    }
                    Err(_t) => {
                        String::from("No such file or directory")
                    }
                };

                if name_clone=="No such file or directory"{
                    println!("No such file or directory");
                }
                else{
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
                    input = input.trim().parse().unwrap();
                    match input.as_ref(){
                        "local file"=>{
                            match fs::remove_dir_all(&name){
                                Ok(_T)=>{},
                                Err(_E)=>{}
                            }
                            fs::rename(name_clone, name).unwrap();
                        }
                        "remote file"=>{
                            let md = metadata(&name_clone).unwrap();
                            if md.is_dir(){
                                file.directory_upload(&path,&name,& stream);
                                fs::remove_dir_all(&name_clone).unwrap();
                            }
                            else if md.is_file(){
                                println!("{}",name);
                                file.file_upload(&name,stream);
                                fs::remove_file(&name_clone).unwrap();
                            }
                        }
                        _=>{
                            let md = metadata(&name_clone).unwrap();
                            if md.is_dir(){
                                fs::remove_dir_all(&name_clone).unwrap();
                            }
                            else if md.is_file(){
                                fs::remove_file(&name_clone).unwrap();
                            }
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
            "exit"=>{
                for mut stream in &self.vec_stream{
                    stream.write("exit".as_bytes());
                }
                false
            }
            _ => false
        }
    }
}


//Used to storing the multiple operations
#[derive(Serialize, Deserialize, Debug)]
pub struct Operations{
    pub operations: Vec<Operation>,
}
impl Operations {
    pub fn new(file_name : &str ) -> Self {
        let mut file = File::open(file_name)
            .expect("Open file failed");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Read file failed");
        serde_json::from_str(&data).unwrap()
    }
}

//Used to storing the single operations
#[derive(Serialize, Deserialize, Debug)]
pub struct Operation{
    pub para_1: String,
    pub para_2: String,
    pub para_3: String,
    pub group: isize,
    pub node: isize,
}

pub struct OperaStream {
    pub operation: Operation,
    pub vec_stream:Vec<TcpStream>,
}
//Automatically execute the program using the parameters in struct Opertion
impl OperaStream {
    pub fn new(operation:Operation,vec_stream:Vec<TcpStream>)-> Self{
        OperaStream { operation, vec_stream }
    }
    pub fn selected(&mut self){
        if self.operation.para_1=="file" {
            let pos = self.operation.para_3.rfind("=").unwrap();
            let local_file = String::from(&self.operation.para_3[..pos]);
            let remote_file = String::from(&self.operation.para_3[pos + 1..]);
            match self.operation.para_2.as_ref() {
                "upload file" => {
                    let mut local = String::from(&local_file);
                    let mut file = OpFile::new(local);
                    //Check whether the path exists
                    match metadata(&local_file) {
                        Ok(md) => {
                            ////Check whether the path is directory
                            if md.is_dir() {
                                if self.operation.node != -1 {
                                    file.directory_upload(&local_file, &remote_file,
                                                          &self.vec_stream[self.operation.node as usize]);
                                } else {
                                    for stream in &mut self.vec_stream {
                                        file.directory_upload(&local_file, &remote_file, &stream);
                                    }
                                }
                            } else if md.is_file() {
                                if self.operation.node != -1 {
                                    file.file_upload(&remote_file, &self.vec_stream[self.operation.node as usize]);
                                }
                                for stream in &mut self.vec_stream {
                                    file.file_upload(&remote_file, stream);
                                }
                            }
                            println!("Successfully");
                        }
                        Err(_t) => {
                            println!("No such file or directory");
                        }
                    };
                }
                "download file" => {
                    let remote = String::from(&remote_file);
                    let mut file = OpFile::new(remote);
                    let stream = &self.vec_stream[self.operation.node as usize];
                    file.file_download(&local_file, stream);
                }
                "compare file" => {
                    let remote = String::from(&remote_file);
                    let mut file = OpFile::new(remote);
                    //Check whether the path exists
                    let name_clone = match metadata(&local_file) {
                        Ok(_md) => {
                            file.directory_compare(&local_file,
                                                   &self.vec_stream[self.operation.node as usize])
                        }
                        Err(_t) => {
                            String::from("No such file or directory")
                        }
                    };
                    if name_clone == "No such file or directory" {
                        println!("No such file or directory");
                    } else {
                        let mut diff_file = File::open("compare_file").unwrap();
                        let buffer = BufReader::new(diff_file);
                        //Output the different content between these two files or directories
                        for line in buffer.lines() {
                            println!("{}", line.unwrap());
                        }
                        fs::remove_file("compare_file").unwrap();
                        println!();
                        println!("Do you want to update your local file or remote file(any other input will do nothing)?");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        input = input.trim().parse().unwrap();
                        match input.as_ref() {
                            //Choose this to update local file
                            "local file" => {
                                match fs::remove_dir_all(&local_file) {
                                    Ok(_T) => {},
                                    Err(_E) => {}
                                }
                                fs::rename(name_clone, local_file).unwrap();
                            }
                            //Choose this to update remote file
                            "remote file" => {
                                let md = metadata(&name_clone).unwrap();
                                if md.is_dir() {
                                    file.directory_upload(&local_file, &remote_file,
                                                          &self.vec_stream[self.operation.node as usize]);
                                    fs::remove_dir_all(&name_clone).unwrap();
                                } else if md.is_file() {
                                    file.file_upload(&remote_file,
                                                     &self.vec_stream[self.operation.node as usize]);
                                    fs::remove_file(&name_clone).unwrap();
                                }
                            }
                            _ => {
                                let md = metadata(&name_clone).unwrap();
                                if md.is_dir() {
                                    fs::remove_dir_all(&name_clone).unwrap();
                                } else if md.is_file() {
                                    fs::remove_file(&name_clone).unwrap();
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        //software deployment and system monitoring function used in here.
        else {
            let input = format!("{},{},{}",self.operation.para_1,self.operation.para_2,self.operation.para_3);
            //if the node is -1,means select the all nodes
            if self.operation.node==-1{
                for stream in &mut self.vec_stream{
                    ClientStream::io_stream( stream,&input);
                }
            }
            else{
                ClientStream::io_stream(&mut self.vec_stream[self.operation.node as usize],&input);
            }
        }
        for stream in &mut self.vec_stream{
            stream.write("exit".as_bytes());
        }
    }
}

