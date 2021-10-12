use std::io::prelude::*;
use ssh2::Session;
use std::net::{TcpStream, ToSocketAddrs};

struct Connection
{
    hostname : String,
    username : String,
    password : String,
}

trait ConnectSsh
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

fn main() {
    let conn = Connection{hostname : String::from("hostname:22"),
        username : String::from("username"),
        password : String::from("password")};
    let sess = conn.connect_ssh();

    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());


}
