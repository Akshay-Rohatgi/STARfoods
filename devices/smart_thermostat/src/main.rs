use serde::{ Deserialize, Serialize };
use std::fs::{ File, OpenOptions };
use std::{ env, process };

use simple_crypt::{ encrypt_file, decrypt_file };
use std::path::Path;

use std::io::Read;

const KEY: &str = "SECRET";

#[derive(Debug, Deserialize, Serialize)]
pub struct Thermostat {
    id: i8,
    target_temperature: i32,
    mode: String,
    name: String,
    visible: bool,
    desc: String,
}

fn read_file(path: &String) -> Vec<Thermostat> {
    let file: File = File::open(path).expect("FILE OPEN ERROR");
    let thermos: Vec<Thermostat> = serde_json::from_reader(file).expect("READ ERROR");
    thermos
}

fn write_file(path: &String, thermos: &Vec<Thermostat>) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .expect("OPEN OPTION ERROR");
    serde_json::to_writer(file, thermos).expect("WRITE ERROR");
}

fn encrypt(path: &String) {
    encrypt_file(Path::new(&path), Path::new(&path), KEY.as_bytes()).expect("ENCRYPT ERROR");
}

fn decrypt(path: &String) {
    decrypt_file(Path::new(&path), Path::new(&path), KEY.as_bytes()).expect("DECRYPT ERROR");
}

fn display_thermos(path: &String) {
    let thermos = read_file(&path);
    println!("Building Thermostats");
    for thermo in thermos.iter() {
        if thermo.visible {
            println!(
                "[+] ID: {} | name: {} | mode: {} | target temperature: {}",
                thermo.id,
                thermo.name,
                thermo.mode,
                thermo.target_temperature
            );
            println!("{}", thermo.desc);
            println!("-----------------------------");
        }
    }
}

fn get_thermo(path: &String, id: i8) {
    let thermos: Vec<Thermostat> = read_file(&path);
    if let Some(thermo) = thermos.iter().find(|thermo| thermo.id == id) {
        println!(
            "[+] ID: {} | name: {} | mode: {} | target temperature: {}",
            thermo.id,
            thermo.name,
            thermo.mode,
            thermo.target_temperature
        );
        println!("{}", thermo.desc);
        println!("-----------------------------");
    }
}

fn switch_mode(path: &String, id: i8) {
    let mut thermos: Vec<Thermostat> = read_file(&path);
    // more idiomatic appraoch instead of looping through
    if let Some(thermo) = thermos.iter_mut().find(|thermo| thermo.id == id) {
        thermo.mode = match thermo.mode.as_str() {
            "heating" => "cooling".to_string(),
            "cooling" => "heating".to_string(),
            _ => thermo.mode.clone(),
        };
    }
    write_file(&path, &thermos);
}

fn set_target(path: &String, id: i8, target: i32) {
    let mut thermos = read_file(&path);
    if let Some(thermo) = thermos.iter_mut().find(|thermo| thermo.id == id) {
        thermo.target_temperature = target;
    }
    write_file(&path, &thermos);
}

fn help() {
    println!("[!] Usage: smart_thermostat <operation>");
    println!(
        "[!] Operations: \n 
              [-] display \n
              [-] get <id> \n
              [-] switch_mode <id> \n
              [-] set_target <id> <target_temperature> \n
              "
    );
}

// worst check of all time but I wrote this at 4am
fn is_json_array(file_path: &str) -> bool {
    let mut file = File::open(file_path).expect("FILE READ ERROR");
    let mut first_char = [0u8; 1];

    match file.read_exact(&mut first_char) {
        Ok(_) => first_char[0] == b'[',
        Err(_) => false,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = env::var("DB_PATH").unwrap() + "/thermostat_db.json";

    if args.len() < 2 {
        help();
        process::exit(1);
    }

    if !is_json_array(&path.as_str()) {
        decrypt(&path);
    }

    match args[1].as_str() {
        "e" => {
            if is_json_array(&path.as_str()) {
                encrypt(&path);
                process::exit(0);
            }
        }
        "display" => display_thermos(&path),
        "get" => get_thermo(&path, args[2].parse::<i8>().expect("INVALID ID")),
        "switch_mode" => switch_mode(&path, args[2].parse::<i8>().expect("INVALID ID")),
        "set_target" =>
            set_target(
                &path,
                args[2].parse::<i8>().expect("INVALID ID"),
                args[3].parse::<i32>().expect("INVALID TEMP")
            ),
        _ => help(),
    }

    encrypt(&path)
}
