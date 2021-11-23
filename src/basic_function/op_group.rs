use std::{fs, thread};
use std::fs::File;
use std::io::{Read, Write};
use ssh2::Session;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection
{
    pub hostname : String,
    pub port : Sting,
    pub username : String,
    pub password : String,
}

//This session is the ssh session
pub struct RSession{
    pub sess : Session
}

impl Connection {
    //connected by ssh
    pub fn connect_ssh(&self) ->Session {
        let bind_port = format!("{}{}", self.hostname, ":22");
        let tcp = TcpStream::connect(bind_port).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        sess.userauth_password(&self.username, &self.password).unwrap();
        assert!(sess.authenticated());
        sess
    }
    //connected by TCP stream
    pub fn connect_tcp(&self) -> TcpStream{
        let mut stream = TcpStream::connect(self.hostname.add(self.port)).unwrap();
        stream
    }
}

impl RSession {
    fn new(sess : Session) -> RSession{
        RSession{sess}
    }
    //If the server node don't have the server app, use this function to upload server app.
    fn upload_server_app(self, file_path: &str){

        let data = fs::read(file_path).unwrap();
        let mut server_app = self.sess.scp_send(Path::new(file_path)
                                                , 0o644, data.len() as u64, None).unwrap();
        server_app.write(&data).unwrap();

    }

    //open the server app
    pub fn open_server(self, app_name: &str) {
        let mut channel = self.sess.channel_session().unwrap();
        let mut comm = format!("find / -name {} -type f",app_name, );
        channel.exec(&comm).unwrap();
        let mut str = String::new();
        channel.read_to_string(&mut str).unwrap();
        match str.len() {
            0 => {
                self.upload_server_app(app_name);
                str = format!("./{}",app_name);
            },
            _ => {
                str = str::replace(&str, "\n", "");
            }
        }
        channel.exec(&str).unwrap();
        channel.wait_close().ok();
    }

}


impl Groups{
    pub fn new(file_name : &str ) -> Groups {
        let mut file = File::open(file_name).expect("Open file failed");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Read file failed");
        serde_json::from_str(&data).unwrap()
    }

    //Select group you want to operate by group ID.
    pub fn select_group(self, group_id : usize, name: &str) {
        if self.groups.len()>=group_id {

            let mut thread_handle: Vec<thread::JoinHandle<()>> = Vec::new();
            for conn in &self.groups[group_id]{
                let rsess = RSession::new(conn.connect_ssh());
                rsess.open_server(name);
                //handle_client(stream).unwrap();
                let stream = conn.connect_tcp();

                let handle = thread::spawn(move || {
                    handle_server(stream);
                });
                thread_handle.push(handle);
            }
            for handle in thread_handle {
                handle.join().unwrap();
            }
        }
        else {
            println!("No such group");
        }
    }
}


