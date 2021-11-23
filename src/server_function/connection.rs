use std::net::{TcpListener, TcpStream};

struct ServerInfo{
    port : String
}

impl ServerInfo {
    fn new(port:String) -> ServerInfo{
        ServerInfo{port}
    }

    fn connect_client(&self) -> TcpStream{
        let together = format!("0.0.0.1{}",self.port);
        let listener = TcpListener::bind(together).unwrap();
        listener.incoming().next().unwrap().expect("failed")
    }

}
