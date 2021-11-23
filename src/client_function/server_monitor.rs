use std::io::{Read, Write};
use std::net::TcpStream;

struct MonCPU{
    stream: TcpStream,
}

struct MonMemory{
    stream: TcpStream,
}
impl MonCPU{
    fn input(&mut self, input: String){
        self.stream.write(input.as_bytes()).expect("failed to write");
        let mut server_buf = [0;1024];
        let client_read = self.stream.read(&mut server_buf).unwrap();
        let mut mess = std::str::from_utf8(&server_buf[..client_read-1]).unwrap();
        println!("read form server:{:?}", mess);
        println!();
    }
    fn logical_cpu_number(&mut self){
        MonCPU::input(&mut self,"logical_cpu_number".into_string());
    }
    fn physical_cpu_number(&mut self){
        MonCPU::input(&mut self,"physical_cpu_number".into_string());
    }
    fn cpu_info(&mut self){
        MonCPU::input(&mut self,"cpu_info".into_string());
    }

}

impl MonMemory{
    fn input(&mut self, input: String){
        self.stream.write(input.as_bytes()).expect("failed to write");
        let mut server_buf = [0;1024];
        let client_read = self.stream.read(&mut server_buf).unwrap();
        let mut mess = std::str::from_utf8(&server_buf[..client_read-1]).unwrap();
        println!("read form server:{:?}", mess);
        println!();
    }

    fn men_state(&mut self){
        MonMemory::input(&mut self,"memory_info".into_string());
    }

    fn men_free(&mut self){
        MonMemory::input(&mut self,"memory_free".into_string());
    }

}



