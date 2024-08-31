use std::env;
use std::fs;
use std::process::Command;
use std::error::Error;

fn apply_firmware_update(path: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("/bin/bash")
        .arg(path)
        .output()?;

    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        eprintln!("Error:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        return Err(format!("Firmware update failed: {:?}", output).into());
    }

    println!("Firmware update applied successfully.");
    Ok(())
}

fn list_items(items_file: &str) -> Result<(), Box<dyn Error>> {
    let items_content = fs::read_to_string(items_file)?;

    println!("Items in the fridge:");
    for line in items_content.lines() {
        println!("{}", line);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [<script_path>]", args[0]);
        std::process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "update" => {
            if args.len() != 3 {
                eprintln!("Usage: {} update <firmware_file>", args[0]);
                std::process::exit(1);
            }
            let script_path = &args[2];
            apply_firmware_update(script_path)?;
        },
        "list" => {
            let path = env::var("DB_PATH").unwrap() + "/items.txt";
            let items_file = path.as_str();
            list_items(items_file)?;
        },
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Usage: {} <command> [<script_path>]", args[0]);
            std::process::exit(1);
        },
    }

    Ok(())
}
