use ssh2::Session;
use std::io::prelude::*;

pub struct Command
{
    pub sess : Session,
    pub comm : String,
}

pub trait  ExCommand
{
    fn ex_command(&self) ->String;
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