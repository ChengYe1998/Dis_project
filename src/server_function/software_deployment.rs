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
        let comm = format!("{} list --showduplicates {}",self.package_tool, self.name);
        self.execute(comm)
    }

    fn query(&mut self) -> String {
        let comm =match self.package_tool.as_ref(){
            "yum"=> format!("{} list installed | grep -i {}",self.package_tool,self.name),
            _ => format!("{} list -- installed | grep -i {}",self.package_tool,self.name)
        };
        let result = self.execute(comm);
        let status = match result.len(){
            0=>"Not installed",
            _=>"Already installed"
        };
        String::from(status)
    }

    fn install(&mut self) -> String {
        match self.query().as_ref(){
            "Not installed"=>{
                let comm= match self.version.as_ref() {
                    ""=>format!("{} install -y {}",self.package_tool,self.name),
                    _=>format!("{} install -y {}-{}",self.package_tool,self.name,self.version)
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

    fn install_by_version(&mut self, server_stream: &mut ServerStream) -> String {
        let result = self.software_version();
        server_stream.stream.write(result.as_bytes()).unwrap();
        self.version=server_stream.read_client();
        self.install()
    }

    fn update(&mut self) -> String {
        let comm =  format!("{} update -y {}",self.package_tool,self.name);
        let result = self.execute(comm);
        let find_result =  match result.find("No packages marked for update")
        {
            Some(_v)=> "Update successfully",
            None=> "Nothing need to update"
        };
        String::from(find_result)
    }

    fn remove(&mut self) -> String {
        let comm = format!("{} remove -y {}*", self.package_tool, self.name);
        let result = self.execute(comm);
        result
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