use std::process::Command;


struct SoftwareInfo{
    name: String,
    version: String,
    package_tool: String
}

impl SoftwareInfo {
    fn new(name:String) -> SoftwareInfo{
        let package_tool = String::from(SoftwareInfo::choose_tool());
        let version = String::from("");
        SoftwareInfo{name,version,package_tool}
    }
    fn choose_tool() -> &'static str {
        let excu = Command::new("sh").arg("-c").arg("apt").output().expect("sh exec error!");
        let result = std::str::from_utf8(&excu.stdout).unwrap();
        let find_result = result.find("command not found").unwrap();
        let str = match find_result {
            None => "apt",
            _ => "yum"
        };
        str
    }

    pub fn software_version(&self) -> String{
        let comm = format!("{} list --showduplicates {}",self.package_tool, self.name);
        let excu = Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        let result = std::str::from_utf8(&excu.stdout).unwrap();
        String::from(result)
    }

    pub fn installed(&self) -> &str{
        let comm =match self.package_tool{
            String::from("yum") => format!("{} list installed | grep {}",self.package_tool,self.name),
            _ => format!("{} list -- installed | grep {}",self.package_tool,self.name)
        };
        let excu = Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        let result = std::str::from_utf8(&excu.stdout).unwrap();
        let status = match result.len(){
            0=>"Not installed",
            _=>"Already installed"
        };
        status
    }

    pub fn install(&self) -> String{

        let mut comm = format!("{} install -y {}",self.package_tool,self.name);
        //let excu =
        Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        //let result = std::str::from_utf8(&excu.stdout).unwrap();
        let status = String::from(match installed(&self.name){
            "Already installed" => "installed successfully",
            _ => "installed failed"
        });
        status
    }

    pub fn update(&self) -> String{

        let mut comm =  format!("{} update -y {}",self.package_tool,self.name);
        let excu = Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        let mut result = String::from(std::str::from_utf8(&excu.stdout).unwrap());
        println!("{}", result);
        result
    }

    pub fn remove(&self) -> String {
        let mut comm = format!("{} remove -y {}*", self.package_tool, self.name);
        let excu = Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        let result = String::from(std::str::from_utf8(&excu.stdout).unwrap());
        println!("{}", result);
        result
    }
}