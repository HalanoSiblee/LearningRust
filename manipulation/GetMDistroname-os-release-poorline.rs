//Let's get my distro name - I use Arch btw   
use std::fs;

fn main() {
    let content = fs::read_to_string("/etc/os-release").expect("The file not exist are you using linux ? 👀");
    
    for line in content.lines() {
        if let Some(index) = line.find("NAME=") {
            let extracted = &line[index + 5..];
            println!("{}", extracted);
        }
    }
}
