mod function;

use ssh2::Session;
use std::io::prelude::*;

use crate::function::connect::Connection;
use crate::function::connect::ConnectSsh;
use crate::function::command_used::Command;
use crate::function::command_used::ExCommand;

fn main()  {
    let conn = Connection{hostname : String::from("hostname:22"),
        username : String::from("username"),
        password : String::from("password")};
    let exec = Command{sess: Session::from(conn.connect_ssh()),
        comm: String::from("command")};
    let s = exec.ex_command();
    print!("{}",s);


}
