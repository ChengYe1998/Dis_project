use std::io::prelude::*;
use ssh2::Session;
use std::net::{TcpStream, ToSocketAddrs};
use std::fs::File;
use std::fs;
use std::io::prelude::*;

struct Connection
{
    hostname : String,
    username : String,
    password : String,
}

struct Command
{
    sess : Session,
    comm : String,
}

struct NewFile
{
    filename: String,
}

trait ConnectSsh
{
    fn connect_ssh(&self) ->Session;
}

trait  ExCommand
{
    fn ex_command(&self) ->String;
}

trait  CreateFile
{
    fn create_file(&self) -> std::io::Result<()>;
}

impl CreateFile for NewFile {
    fn create_file(&self) -> std::io::Result<()> {
        File::create(&self.filename)?;
        Ok(())
    }
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

impl ExCommand for Command {
    fn ex_command(&self) -> String {
        let mut channel = self.sess.channel_session().unwrap();
        channel.exec(&self.comm).unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        channel.wait_close();
        s
    }
}

fn main() {
    let conn = Connection{hostname : String::from("hostname:22"),
        username : String::from("username"),
        password : String::from("password")};

    let sess = conn.connect_ssh();
    let exec = Command{sess: Session::from(sess),
        comm: String::from("ls")};
    let s = exec.ex_command();
    println!("{}", s);

    let newfile = NewFile{filename : String::from("test1.txt")};
    newfile.create_file();


}
