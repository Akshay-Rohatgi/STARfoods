use serde_derive::{Deserialize, Serialize};
use simple_crypt::{encrypt_file, decrypt_file};
use serde_xml_rs::from_str;
use std::env;
use std::fs::{File, copy};
use std::io::{Read, Write};
use std::path::Path;

const VERSION: &str = "1.7.22"; 
const KEY: &str = "SECRET";

#[derive(Debug, Deserialize, Serialize)]
struct AirPurifierConfig {
    major_version: String,
    fan_speed: String,
    filter_replacement_schedule: String,
    air_quality_threshold: i32,
}

fn display(path: &str) {
    let mut file = File::open(path).expect("FAILED TO OPEN CONFIGURATION");
    let mut xml_data = String::new();
    file.read_to_string(&mut xml_data).expect("FAILED TO READ CONFIG");
    let config: AirPurifierConfig = from_str(&xml_data).expect("FAILED TO PARSE CONFIG");

    println!("Current ~QUASAIR~ Configuration:");
    println!("================================");
    println!("Major Version: {}", config.major_version);
    println!("Fan Speed: {}", config.fan_speed);
    println!("Filter Replacement Schedule: {}", config.filter_replacement_schedule);
    println!("Air Quality Threshold: {}", config.air_quality_threshold);
    println!("================================");
}

fn update(config_path: &str, new_config_path: &str, debug: bool) {
    let mut file = File::open(new_config_path).expect("FAILED TO OPEN NEW CONFIGURATION");
    let mut xml_data = String::new();
    file.read_to_string(&mut xml_data).expect("FAILED TO READ NEW CONFIG");
    let new_config: AirPurifierConfig = from_str(&xml_data).expect("FAILED TO PARSE NEW CONFIG");

    if new_config.major_version.split('.').next().unwrap() != VERSION.split('.').next().unwrap() {
        if debug {
            eprintln!("Version mismatch; this is QUASAIR version {}", VERSION);
        } else {
            eprintln!("Invalid configuration");
        }
        return;
    }

    copy(new_config_path, config_path).expect("FAILED TO UPDATE CONFIGURATION");

    println!("Configuration updated successfully.");
}

fn encrypt(path: &str) {
    encrypt_file(Path::new(path), Path::new(path), KEY.as_bytes()).expect("ENCRYPT ERROR");
}

fn decrypt(path: &str) {
    decrypt_file(Path::new(path), Path::new(path), KEY.as_bytes()).expect("DECRYPT ERROR");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = env::var("DB_PATH").unwrap() + "/config.xml";

    decrypt(&path);

    if args.len() >= 2 {
        match args[1].as_str() {
            "display" => display(&path),
            "update" => {
                if args.len() < 3 {
                    eprintln!("Usage: {} update <path_to_new_config> [--debug]", args[0]);
                    return;
                }
                let new_config_path = &args[2];
                let debug = args.len() > 3 && args.contains(&"--debug".to_string());
                update(&path, new_config_path, debug);
            }
            _ => {
                eprintln!("Usage: {} [display|update <path_to_new_config> [--debug]]", args[0]);
            }
        }
    } else {
        eprintln!("Usage: {} [display|update <path_to_new_config> [--debug]]", args[0]);
    }

    encrypt(&path);
}
