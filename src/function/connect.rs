use ssh2::Session;
use std::net::{TcpStream, ToSocketAddrs};

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
        //let host_ip: Vec<_> = self.hostname.to_socket_addrs().unwrap().collect();
        let tcp = TcpStream::connect(&self.hostname).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();

        sess.userauth_password(&*self.username, &*self.password).unwrap();
        assert!(sess.authenticated());
        sess
    }

}




