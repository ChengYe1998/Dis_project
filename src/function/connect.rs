use ssh2::Session;
use std::net::{TcpStream, ToSocketAddrs};
use structopt::StructOpt;
extern crate serde;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Groups{
    pub groups: Vec<Vec<Connection>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection
{
    pub hostname : String,
    pub username : String,
    pub password : String,
}

pub trait ConnectSsh
{
    fn connect_ssh(&self) ->Session;
}

impl ConnectSsh for Connection {
    fn connect_ssh(&self) ->Session {
        let tcp = TcpStream::connect(&self.hostname).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();

        sess.userauth_password(&self.username, &self.password).unwrap();
        assert!(sess.authenticated());
        sess
    }

}




