use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, Write};

fn generate_otp() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let time_in_seconds = since_the_epoch.as_secs();
    (time_in_seconds % 10000) as u32
}

fn verify_otp(otp: u32, input: u32) -> bool {
    for offset in 0..=2 {
        let past_otp = (otp + offset) % 10000;
        let future_otp = (otp + 10000 - offset) % 10000;
        if input == past_otp || input == future_otp {
            return true;
        }
    }

    false
}

fn main() {
    let current_otp = generate_otp();
    print!("Please enter the MFA token: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input: u32 = input.trim().parse().expect("Please enter a valid number");

    if verify_otp(current_otp, input) {
        println!("MFA verification successful.");
        println!("PULSAR MFA      ============================================");
        println!("Serial: 8671031 ============================================");
        println!("============================================================");
        println!("Stock Configuration, view this device's documentation here: ");
    } else {
        println!("MFA verification for PULSAR MFA failed. Access denied.");
    }
}
