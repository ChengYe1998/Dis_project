use std::{fs, process, thread};
use std::fs::File;
use std::io::{Read, Write};
use ssh2::{Channel, Session};
use std::net::{TcpStream, ToSocketAddrs};
use std::ops::Add;
use std::path::Path;
use structopt::StructOpt;
extern crate serde;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Groups{
    pub groups: Vec<Vec<Connection>>,
}
impl Groups{
    pub fn new(file_name : &str ) -> Self {
        let mut file = File::open(file_name)
            .expect("Open file failed");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Read file failed");
        serde_json::from_str(&data).unwrap()
    }
    //Select group you want to operate by group ID.
    pub fn select_group(self, group_id : usize, name: &str) -> Vec<TcpStream>{
        let n= self.groups.len();
        let mut vec_stream:Vec<TcpStream> = vec![];
        if n > group_id
        {
            for conn in &self.groups[group_id] {
                let rsess = RSession::new(conn.connect_ssh());
                let mut stream;
                loop{
                    rsess.open_server(name);
                    stream = match conn.connect_tcp(){
                        Ok(t)=>{
                            println!("Connect successfully");
                            t
                        },
                        Err(_e)=>{
                            continue;
                        }
                    };
                    break;
                }
                vec_stream.push(stream);
            }
        }
        vec_stream
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection
{
    pub hostname : String,
    pub port : String,
    pub username : String,
    pub password : String,
}
impl Connection {
    pub fn new(hostname:String, port:String,username:String,password:String)->Self{
        Connection{hostname, port, username,password}
    }
    //connected by ssh
    pub fn connect_ssh(&self) ->Session {
        let bind_port = format!("{}{}", self.hostname, ":22");
        let tcp = TcpStream::connect(bind_port)
            .expect("connect to the remote server failed, please check the information of the server");

        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake()
            .expect("Failed to establish connection");
        sess.userauth_password(&self.username, &self.password)
            .expect("Please check the username and password");
        assert!(sess.authenticated());

        sess
    }
    //connected by TCP stream
    pub fn connect_tcp(&self) -> Result<TcpStream,  &'static str>{
        let together = format!("{}:{}",&self.hostname,&self.port);
        let stream = match  TcpStream::connect(together){
            Ok(T)=>T,
            Err(_E)=>return Err("Connect failed"),
        };
        Ok(stream)
    }
}

//This session is the ssh session
pub struct RSession{
    pub sess : Session
}
impl RSession {
    fn new(sess : Session) -> Self{
        RSession{sess}
    }
    //If the server node don't have the server app, use this function to upload server app.
    fn upload_server_app(&self, file_path: &str) {

        let data = fs::read(file_path)
            .expect("read file failed");

        let mut server_app = self.sess.scp_send(Path::new(file_path)
                                                ,0o644,data.len() as u64, None)
            .expect("open channel failed");

        server_app.write(&data)
            .expect("write data failed");
    }
    //open the server app
    pub fn open_server(&self, app_name: &str) {
        let mut channel = self.sess.channel_session()
            .expect("open channel failed");
        let mut comm = format!("find -name {} -type f",app_name);
        channel.exec(&comm)
            .expect("command executed failed");
        let mut result = String::new();
        channel.read_to_string(&mut result).unwrap();
        channel.close();
        match result.len() {
            0 => {
                self.upload_server_app(app_name);
                result = format!("./{} &",app_name);
            },
            _ => {
                result = str::replace(&result, "\n", " &");
            }
        }

        let mut channel = self.sess.channel_session()
            .expect("open channel failed");
        channel.exec(&result)
            .expect("command executed failed");
        channel.close();

    }
}

