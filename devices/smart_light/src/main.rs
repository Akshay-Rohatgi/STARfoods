use md5;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::process;

const HASH: &str = "7315ac7ba3b60a5b053886fa49f98ed6";

#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    on: bool,
    name: String,
}

fn toggle() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::var("DB_PATH").unwrap() + "/light_status.json";
    let file: File = File::open(&path)?;
    let mut status: Status = serde_json::from_reader(file)?;

    status.on = !&status.on;
    let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)?;

    serde_json::to_writer(file, &status)?;

    println!("Status toggled and updated in file: {:?}", &status);
    Ok(())

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("[!] Usage: smart_light <password>");
        process::exit(1);
    }

    let check = md5::compute(&args[1].as_bytes());

    if format!("{:x}", check) == HASH {
        println!("[+] Password accepted, toggling light!");
        if let Err(e) = toggle() {
            eprintln!("Error in toggle function: {}", e);
        }
    } else {
        println!("[-] Incorrect password");
    }
}
