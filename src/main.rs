mod basic_function;
mod client_function;
mod server_function;


use std::fs::File;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use crate::server_function::op_stream::ServerStream;
use std::{fs, io};
use std::io::Read;
use std::path::Path;
use crate::basic_function::op_group::{RSession, Connection, Groups};
use crate::client_function::op_stream::ClientStream;
use crate::client_function::server_monitor::{MonServer, MonType};
use crate::client_function::software_deployment::SoftDeploy;

//for client test
/*
fn main() {
    let gs = Groups::new("config.json");
    let vec = gs.select_group(0,"rust_server_app");
    println!("Which function you want to use:");
    let mut function = String::new();
    io::stdin().read_line(&mut function).expect("Failed to read line");
    let mut function = String::from(&function).replace("\n","");
    let mut client_stream = ClientStream::new(vec,function);
    while client_stream.select_function()==false{
        println!("Please choose the right function:");
        function = String::new();
        io::stdin().read_line(&mut function).expect("Failed to read line");
        function = String::from(&function).replace("\n","");
        client_stream.function=function;
    }
}*/


//for test server side
fn main(){
    let mut stream = ServerStream::new();
    stream.select();
}
