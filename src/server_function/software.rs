use std::process::Command;

//search the system version of the linux server
fn search_version() -> &'static str {
    let mut comm = "grep -E -i \"red|cent|fed\" /proc/version";
    let excu = Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
    let result = std::str::from_utf8(&excu.stdout).unwrap();
    let str = match result.len() {
        0 => "apt",
        _ => "yum"
    };
    str
}

//search the software is already installed or not
pub fn installed(name: &str) -> &str{
    let tool = search_version();
    if tool=="yum" {
        let mut comm = format!("{}{}","yum list installed | grep ",name);
        let excu = Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        let result = std::str::from_utf8(&excu.stdout).unwrap();
        let status = match result.len(){
            0=>"Not installed",
            _=>"Already installed"
        };
        status

    }
    else {
        let mut comm = format!("{}{}","apt list --installed ｜ grep ",name);
        let excu = Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        let result = std::str::from_utf8(&excu.stdout).unwrap();
        let status = match result.len(){
            0=>"Not installed",
            _=>"Already installed"
        };
        status
    }
}

//install software
pub fn install(name: &str) -> String{

    let tool = search_version();
    if tool=="yum" {

        let mut comm = format!("{}{}","yum install -y ",name);
        Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        //let result = std::str::from_utf8(&excu.stdout).unwrap();
        let status = String::from(match installed(name){
            "Already installed" => "installed successfully",
            _ => "installed failed"
        });
        status
    }
    else {
        let mut comm = format!("{}{}","apt install -y ",name);
        Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        //let result = std::str::from_utf8(&excu.stdout).unwrap();
        let status = String::from(match installed(name){
            "Already installed"=>"installed successfully",
            _=>"installed failed"
        });
        status
    }

}

//update software
pub fn update(name: &str) -> String{

    let tool = search_version();
    if tool=="yum" {
        let mut comm =  format!("{}{}","yum update -y ",name);
        let excu = Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        let mut result = String::from(std::str::from_utf8(&excu.stdout).unwrap());
        println!("{}", result);
        result

    }
    else {
        let mut comm =  format!("{}{}","apt update -y ",name);
        let excu = Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        let result = String::from(std::str::from_utf8(&excu.stdout).unwrap());
        println!("{}", result);
        result
    }
}

//remove software
pub fn remove(name: &str) -> String{

    let tool = search_version();
    if tool=="yum" {
        let mut comm =  format!("{}{}{}","yum remove -y ",name,"*");
        let excu = Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        let result = String::from(std::str::from_utf8(&excu.stdout).unwrap());
        println!("{}", result);
        result

    }
    else {
        let mut comm =  format!("{}{}","apt remove -y ",name);
        let excu = Command::new("sh").arg("-c").arg(comm).output().expect("sh exec error!");
        let result = String::from(std::str::from_utf8(&excu.stdout).unwrap());
        println!("{}", result);
        result
    }
}