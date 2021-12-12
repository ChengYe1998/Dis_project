use std::io::Write;
use std::process::Command;
use crate::server_function::op_stream::ServerStream;

pub struct SoftwareInfo{
    pub name: String,
    version: String,
    package_tool: String,
}
impl SoftwareInfo {
    pub fn new(name:String, version:String) -> SoftwareInfo{
        let package_tool = String::from(SoftwareInfo::choose_tool());
        let version = version;
        SoftwareInfo{name,version,package_tool}
    }
    fn execute(&mut self, comm: String) -> String{
        let excu = Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        let result = String::from(std::str::from_utf8(&excu.stdout).unwrap());
        result
    }
    fn choose_tool() -> &'static str {
        let result = match Command::new("apt").output(){
            Ok(..) => "apt",
            Err(..) => "yum"
        };
        result
    }
    fn software_version(&mut self) -> String {
        let result =match self.package_tool.as_ref() {
            "yum"=> {
                let comm = format!("yum list --showduplicates {}", self.name);
                let result = self.execute(comm);
                match result.find(&self.name){
                    Some(_v)=>result,
                    None=>String::from(""),
                }
            },
            _=>{
                let comm = format!("apt-cache showpkg {}",self.name);
                self.execute(comm)
            },
        };
        result
    }
    fn query(&mut self) -> String {
        if self.software_version().len()==0{
            String::from("Software not exists")
        }
        else{
            let comm =match self.package_tool.as_ref(){
                "yum"=> format!("yum list installed | grep -i {}",self.name),
                _ => format!("apt list -- installed | grep -i {}",self.name)
            };
            let result = self.execute(comm);
            let status = match result.len(){
                0=>"Not installed",
                _=>"Already installed"
            };
            String::from(status)
        }
    }
    fn install(&mut self) -> String {
        if self.software_version().len()==0{
            String::from("Software not exists")
        }
        else{
            match self.query().as_ref(){
                "Not installed"=>{
                    let comm= match self.version.as_ref() {
                        ""=>{
                            match self.package_tool.as_ref(){
                                "yum"=>format!("yum install -y {}",self.name),
                                _=>format!("sudo apt install -y {}",self.name),
                            }

                        },
                        _=> {
                            match self.package_tool.as_ref(){
                                "yum"=> format!("yum install -y {}-{}", self.name, self.version),
                                _=> format!("sudo apt install -y {}={}", self.name, self.version)
                            }
                        }
                    };
                    self.execute(comm);
                    let status = match self.query().as_ref(){
                        "Already installed" => "Installed successfully",
                        _ => "Installed failed"
                    };
                    String::from(status)
                }
                _=>String::from("Already installed")
            }
        }
    }
    fn install_by_version(&mut self, server_stream: &mut ServerStream) -> String {
        //get the version info for the software
        let result = self.software_version();
        if result.len()==0{
            String::from("Software not exists")
        }
        else{
            //tell the version info to client
            server_stream.stream.write(result.as_bytes()).unwrap();
            //get client's selected
            self.version=server_stream.read_client();
            self.install()
        }
    }
    fn update(&mut self) -> String {
        if self.software_version().len()==0{
            String::from("Software not exists")
        }
        else{
            let comm = match self.package_tool.as_ref() {
                "yum"=>format!("yum update -y {}",self.name),
                _=>format!("sudo apt-get upgrade -y {}",self.name),
            };
            let result = self.execute(comm);
            let find_result =  match result.find("No packages marked for update")
            {
                Some(_v)=> "Nothing need to update",
                None=> "Update successfully"
            };
            String::from(find_result)
        }
    }
    fn remove(&mut self) -> String {
        if self.software_version().len()==0{
            String::from("Software not exists")
        }
        else{
            let comm = format!("{} remove -y {}*", self.package_tool, self.name);
            let result = self.execute(comm);
            result
        }
    }
    pub fn selected(&mut self, method:String, server_stream:&mut ServerStream) -> String{
        match method.as_ref() {
            "query" => {
                let result = self.query();
                result
            },
            "install" =>{
                let result = self.install();
                result
            },
            "update" => {
                let result = self.update();
                result
            },
            "remove" => {
                let result = self.remove();
                result
            },
            "version" => {
                let result = self.install_by_version(server_stream);
                result
            },
            _ => String::from("Method doesn't exist"),
        }
    }
}

