use std::env;
use std::process;

fn text_to_hex(text: &str) -> String {
    text.as_bytes().iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join("")
}

fn hex_to_text(hex: &str) -> Result<String, &'static str> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have an even length");
    }
    let mut bytes = Vec::new();
    for i in (0..hex.len()).step_by(2) {
        let byte_str = &hex[i..i + 2];
        match u8::from_str_radix(byte_str, 16) {
            Ok(byte) => bytes.push(byte),
            Err(_) => return Err("Invalid hex string"),
        }
    }
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

fn print_usage(program: &str) {
    println!("Usage: {} [to-hex|to-text] <input>", program);
    println!("  to-hex: Convert text to hexadecimal");
    println!("  to-text: Convert hexadecimal to text");
    println!("Example:");
    println!("  {} to-hex \"Hello\"", program);
    println!("  {} to-text \"48656c6c6f\"", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let command = &args[1];
    let input = &args[2];

    match command.as_str() {
        "to-hex" => {
            let result = text_to_hex(input);
            println!("Hex: {}", result);
        }
        "to-text" => match hex_to_text(input) {
            Ok(result) => println!("Text: {}", result),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },
        _ => {
            eprintln!("Error: Invalid command '{}'", command);
            print_usage(&args[0]);
            process::exit(1);
        }
    }
}