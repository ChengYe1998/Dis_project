use std::process::Command;

pub enum MonType{
    CPU,
    Memory,
    Network,
    System,
    User
}

pub struct MonServer{
    mon_type:MonType,
    option:String
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

pub trait Monitor{
    //This function is for select the correct command to response.
    fn selected(&mut self) -> String;
    //Execute the command
    fn execute(&mut self, comm:String) ->String{
        let excu = Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        let result = String::from(std::str::from_utf8(&excu.stdout).unwrap());
        result
    }
}

impl MonServer {
    pub fn new(m_type:String, option:String)->MonServer {
        match m_type.as_ref(){
            "cpu" =>MonServer{mon_type:MonType::CPU, option },
            "mem" =>MonServer{mon_type:MonType::Memory, option },
            "sys" =>MonServer{mon_type:MonType::System, option},
            "user" =>MonServer{mon_type:MonType::User,option},
            _=>MonServer{mon_type:MonType::Network,option}
        }
    }
    pub fn select(&mut self)->String{
        let option = String::from(&self.option);
        let result = match self.mon_type {
            MonType::CPU=>{
                let mut mon_cpu =MonCPU::new(option);
                mon_cpu.selected()
            },
            MonType::Memory=>{
                let mut mon_memory =MonMemory::new(option);
                mon_memory.selected()
            },
            MonType::System=>{
                let mut mon_sys=MonSystem::new(option);
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
        };
        result
    }
}

impl Monitor for MonCPU {
    fn selected(&mut self) -> String {
        match self.option.as_ref(){
            "logical"=>self.logical_cpu_number(),
            "physical"=>self.physical_cpu_number(),
            "detail"=>self.cpu_info(),
            _=>String::from("Option doesn't exist!"),
        }
    }
}

impl MonCPU {
    pub fn new(option:String)->MonCPU{
        MonCPU{option}
    }
    fn logical_cpu_number(&mut self)->String{
        let comm = format!("cat /proc/cpuinfo | grep 'processor' | wc -l");
        self.execute(comm)
    }
    fn physical_cpu_number(&mut self)->String{
        let comm = format!("cat /proc/cpuinfo | grep 'physical id' | sort | uniq | wc -l");
        self.execute(comm)
    }
    fn cpu_info(&mut self)->String{
        let comm = format!("cat /proc/cpuinfo");
        self.execute(comm)
    }
}

impl Monitor for MonMemory {
    fn selected(&mut self) -> String {
        match self.option.as_ref(){
            "usage"=>self.men_usage(),
            "free"=>self.men_free(),
            "total"=>self.men_total(),
            _=>String::from("Option doesn't exist!"),
        }
    }
}

impl MonMemory{
    pub fn new(option :String)->MonMemory{
        MonMemory{option}
    }
    fn men_usage(&mut self)->String{
        let comm = format!("free -m");
        self.execute(comm)
    }
    fn men_free(&mut self)->String{
        let comm = format!("grep MemFree /proc/meminfo");
        self.execute(comm)
    }
    fn men_total(&mut self)->String{
        let comm = format!("grep MemFree /proc/meminfo");
        self.execute(comm)
    }
}

impl Monitor for MonSystem {
    fn selected(&mut self)-> String{
        match self.option.as_ref() {
            "sys_info" => self.basic_info(),
            "kernel_module" => self.loaded_kernel_module(),
            "env"=> self.environment_variable(),
            _=>String::from("Option doesn't exist!"),
        }
    }
}

impl MonSystem{
    pub fn new(option:String)->MonSystem{
        MonSystem{option}
    }
    fn basic_info(&mut self)->String{
        let comm = format!("uptime");
        self.execute(comm)
    }
    fn loaded_kernel_module(&mut self)->String{
        let comm = format!("lsmod");
        self.execute(comm)
    }
    fn environment_variable(&mut self)->String{
        let comm = format!("env");
        self.execute(comm)
    }
}

impl Monitor for MonNetwork {
    fn selected(&mut self)-> String{
        match self.option.as_ref() {
            "interface" => self.network_interfaces(),
            "firewall" => self.firewall_settings(),
            "routing"=>self.routing_table(),
            "connection"=>self.established_connections(),
            "lis_ports"=>self.listening_ports(),
            "net_info"=>self.network_info(),
            _=>String::from("Option doesn't exist!"),
        }
    }
}

impl MonNetwork{
    pub fn new(option:String)->MonNetwork{
        MonNetwork{option}
    }
    fn network_interfaces(&mut self)->String{
        let comm = format!("ifconfig");
        self.execute(comm)
    }
    fn firewall_settings(&mut self)->String{
        let comm = format!("iptables -L");
        self.execute(comm)
    }
    fn routing_table(&mut self)->String{
        let comm = format!("route -n ");
        self.execute(comm)
    }
    fn listening_ports(&mut self)->String{
        let comm = format!("netstat -lntp");
        self.execute(comm)
    }
    fn established_connections(&mut self)->String{
        let comm = format!("netstat -antp");
        self.execute(comm)
    }
    fn network_info(&mut self)->String{
        let comm = format!("netstat -s");
        self.execute(comm)
    }
}

impl Monitor for MonUser {
    fn selected(&mut self)-> String {
        match self.option.as_ref() {
            "active" => self.active_users(),
            "log" => self.user_login_log(),
            "groups" => self.all_groups(),
            "users" => self.all_users(),
            _=>String::from("Option doesn't exist!")
        }
    }
}

impl MonUser {
    pub fn new(option:String)->MonUser{
        MonUser{option}
    }
    fn active_users(&mut self)->String{
        let comm = format!("w");
        self.execute(comm)
    }
    fn user_login_log(&mut self)->String{
        let comm = format!("last");
        self.execute(comm)
    }
    fn all_users(&mut self)->String{
        let comm = format!("cut -d: -f1 /etc/passwd");
        self.execute(comm)
    }
    fn all_groups(&mut self)->String{
        let comm = format!("cut -d: -f1 /etc/group");
        self.execute(comm)
    }
}