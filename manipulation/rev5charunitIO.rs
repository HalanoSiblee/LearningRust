/*
 Okay halano it's time for turning on your eepy brain
 let > 
 Ok(()) is a rust thing that for error handling
 it's like return true in C 
 and Err() return error idk
 let mut > now we declare mutable variable which a var keep changing it's value oppsite of immutable
 then we use std::fs and io API to handling read from the buffer
 then we print it.
*/
use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("no input bro 😭 : ");
        std::process::exit(1);
    }

    let file_name = &args[1];
    let mut file = File::open(file_name)?;

    let mut buffer = [0; 20];
    file.read_exact(&mut buffer)?; //It's wired code I used to close files in C

    let content = String::from_utf8_lossy(&buffer);
    println!("{}", content);

    Ok(())
}
