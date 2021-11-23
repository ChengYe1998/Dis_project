mod basic_function;
mod client_function;
mod server_function;

use ssh2::Session;
use std::fs::File;
use std::io::copy;
use std::io::stdout;
use std::io::prelude::*;
use structopt::StructOpt;

use crate::basic_function::connect::Groups;
use crate::basic_function::connect::Connection;
use crate::basic_function::connect::ConnectSsh;
use crate::basic_function::command_used::Command;
use crate::basic_function::command_used::ExCommand;


fn main()  {

}
