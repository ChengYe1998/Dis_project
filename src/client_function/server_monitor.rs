use std::io::{Read, Write};
use std::net::TcpStream;
use crate::client_function::op_stream::ClientStream;

pub enum MonType{
    CPU,
    Memory,
    Network,
    System,
    User
}

pub trait Monitor{
    fn selected(&mut self)-> String;
}

pub struct MonServer{
    mon_type:MonType,
    pub option:String
}
pub struct  MonCPU{
    option:String
}
pub struct  MonMemory{
    option:String
}
pub struct MonNetwork{
    option: String
}
pub struct MonSystem{
    option: String
}
pub struct MonUser{
    option: String
}

impl MonServer {
    pub fn new(m_type:String, option: String)->MonServer {
        match m_type.as_ref(){
            "cpu" =>MonServer{mon_type:MonType::CPU, option },
            "memory" =>MonServer{mon_type:MonType::Memory, option },
            "system" =>MonServer{mon_type:MonType::System, option},
            "network" =>MonServer{mon_type:MonType::Network, option},
            _=>MonServer{mon_type:MonType::User,option}
        }
    }

    pub fn select(&mut self) -> String{
        let option = String::from(&self.option);
        match self.mon_type {
            MonType::CPU=>{
                let mut mon_cpu =MonCPU::new(option);
                mon_cpu.selected()
            },
            MonType::Memory=>{
                let mut mon_memory =MonMemory::new(option);
                mon_memory.selected()
            }
            MonType::System=>{
                let mut mon_sys =MonSystem::new(option);
                mon_sys.selected()
            }
            MonType::Network=>{
                let mut mon_net =MonNetwork::new(option);
                mon_net.selected()
            }
            MonType::User=>{
                let mut mon_user =MonUser::new(option);
                mon_user.selected()
            }
        }
    }
}

impl Monitor for MonCPU {
    fn selected(&mut self)-> String{
        match self.option.as_ref() {
            "logical" => {
                self.logical_cpu_number()
            },
            "physical" => {
                self.physical_cpu_number()
            },
            "detail" => {
                self.cpu_info()
            },
            _=>{
                String::from("Wrong option")
            }
        }
    }
}

impl MonCPU{
    pub fn new(option:String)->MonCPU{
        MonCPU{option}
    }

    fn logical_cpu_number(&mut self) -> String{
        let mut input = format!("mon,cpu,{}",self.option);
        input
    }
    fn physical_cpu_number(&mut self)-> String{
        let mut input = format!("mon,cpu,{}",self.option);
        input
    }
    fn cpu_info(&mut self)-> String{
        let mut input = format!("mon,cpu,{}",self.option);
        input
    }
}

impl Monitor for MonMemory {
    fn selected(&mut self)-> String{
        match self.option.as_ref() {
            "usage" => {
                self.men_usage()
            },
            "free" => {
                self.men_free()
            },
            "total"=>{
                self.men_total()
            }
            _=>{
                String::from("Wrong option")
            }
        }
    }
}

impl MonMemory{
    pub fn new(option:String)->MonMemory{
        MonMemory{option}
    }

    fn men_usage(&mut self)->String{
        let mut input = format!("mon,mem,{}",self.option);
        input
    }

    fn men_free(&mut self)->String{
        let mut input = format!("mon,mem,{}",self.option);
        input
    }
    fn men_total(&mut self)->String{
        let mut input = format!("mon,mem,{}",self.option);
        input
    }
}

impl Monitor for MonSystem {
    fn selected(&mut self) -> String{
        match self.option.as_ref() {
            "sys_info" => {
                self.basic_info()
            },
            "kernel_module" => {
                self.loaded_kernel_module()
            },
            "env"=>{
                self.environment_variable()
            }
            _=>{
                String::from("Wrong option")
            }
        }
    }
}

impl MonSystem{
    pub fn new(option:String)->MonSystem{
        MonSystem{option}
    }

    fn basic_info(&mut self)->String{
        let mut input = format!("mon,sys,{}",self.option);
        input
    }
    fn loaded_kernel_module(&mut self)->String{
        let mut input = format!("mon,sys,{}",self.option);
        input
    }
    fn environment_variable(&mut self)->String{
        let mut input = format!("mon,sys,{}",self.option);
        input
    }
}

impl Monitor for MonNetwork {
    fn selected(&mut self)-> String{
        match self.option.as_ref() {
            "interface" => {
                self.network_interfaces()
            },
            "firewall" => {
                self.firewall_settings()
            },
            "routing"=>{
                self.routing_table()
            }
            "connection"=>{
                self.established_connections()
            }
            "lis_ports"=>{
                self.listening_ports()
            }
            "net_info"=>{
                self.network_info()
            }
            _=>{
                String::from("Wrong option")
            }
        }
    }
}

impl MonNetwork{
    pub fn new(option:String)->MonNetwork{
        MonNetwork{option}
    }

    fn network_interfaces(&mut self)->String{
        let mut input = format!("mon,net,{}",self.option);
        input
    }

    fn firewall_settings(&mut self)->String{
        let mut input = format!("mon,net,{}",self.option);
        input
    }
    fn routing_table(&mut self)->String{
        let mut input = format!("mon,net,{}",self.option);
        input
    }
    fn listening_ports(&mut self)->String{
        let mut input = format!("mon,net,{}",self.option);
        input
    }
    fn established_connections(&mut self)->String{
        let mut input = format!("mon,net,{}",self.option);
        input
    }
    fn network_info(&mut self)->String{
        let mut input = format!("mon,net,{}",self.option);
        input
    }

}

impl Monitor for MonUser {
    fn selected(&mut self)-> String {
        match self.option.as_ref() {
            "active" => {
                self.active_users()
            },
            "log" => {
                self.user_login_log()
            },
            "groups" => {
                self.all_groups()
            }
            "users" => {
                self.all_users()
            }

            _ => {
                String::from("Wrong option")
            }
        }
    }
}

impl MonUser {
    pub fn new(option:String)->MonUser{
        MonUser{option}
    }

    fn active_users(&mut self)->String{
        let mut input = format!("mon,user,{}",self.option);
        input
    }

    fn user_login_log(&mut self)->String{
        let mut input = format!("mon,user,{}",self.option);
        input
    }
    fn all_users(&mut self)->String{
        let mut input = format!("mon,user,{}",self.option);
        input
    }
    fn all_groups(&mut self)->String{
        let mut input = format!("mon,user,{}",self.option);
        input
    }
}

