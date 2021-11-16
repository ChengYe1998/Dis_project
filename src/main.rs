mod function;
mod client_function;
mod server_function;

use ssh2::Session;
use std::fs::File;
use std::io::copy;
use std::io::stdout;
use std::io::prelude::*;
use structopt::StructOpt;

use crate::function::connect::Groups;
use crate::function::connect::Connection;
use crate::function::connect::ConnectSsh;
use crate::function::command_used::Command;
use crate::function::command_used::ExCommand;


fn main()  {
    let mut file = File::open("config.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let g: Groups = serde_json::from_str(&data).unwrap();
    //Server 1 in the group 1.
    let conn = &g.groups[0][0];

    let exec: Command = serde_json::from_str(&data).unwrap();
    let s = exec.ex_command(conn.connect_ssh());
    print!("{}",s);
}
