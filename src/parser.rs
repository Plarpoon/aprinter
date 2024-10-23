use crate::manual;
use std::env;

/// Function to parse command-line arguments and return parsed values for server, printer, delay, and file names
pub fn parse_arguments() -> Result<(String, String, u64, Vec<String>), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect(); // Collect all arguments passed to the program

    if args.contains(&String::from("-help")) {
        manual::print_manual();
        std::process::exit(0);
    }

    // Default values for CUPS server, printer name, file names, and delay
    let mut cups_server = "localhost".to_string(); // Default to localhost if not provided
    let mut printer_name = "".to_string(); // Default empty, will select default printer
    let mut file_names: Vec<String> = Vec::new(); // Vector to hold the list of files
    let mut delay = 1u64; // Default delay is 1 second

    let mut i = 1; // Start parsing at argument 1 (skip program name)
    while i < args.len() {
        match args[i].as_str() {
            "-url" => {
                // Parse the CUPS server address after the -url argument
                if i + 1 < args.len() {
                    cups_server = args[i + 1].clone();
                    i += 2;
                } else {
                    return Err("Expected argument after -url".into());
                }
            }
            "-pname" => {
                // Parse the printer name after the -pname argument
                if i + 1 < args.len() {
                    printer_name = args[i + 1].clone();
                    i += 2;
                } else {
                    return Err("Expected argument after -pname".into());
                }
            }
            "-delay" => {
                // Parse the delay after the -delay argument
                if i + 1 < args.len() {
                    let delay_str = &args[i + 1]; // Convert the string argument to u64
                    delay = delay_str
                        .parse::<u64>()
                        .map_err(|_| "Delay must be a valid integer")?;
                    i += 2;
                } else {
                    return Err("Expected integer argument after -delay".into());
                }
            }
            "-files" => {
                // Parse the file names after the -files argument
                if i + 1 < args.len() {
                    file_names = args[i + 1..].to_vec(); // Collect all remaining arguments as file names
                    break;
                } else {
                    return Err("Expected argument after -files".into());
                }
            }
            _ => {
                // If an unknown argument is encountered, return an error
                return Err(format!("Unknown argument: {}", args[i]).into());
            }
        }
    }

    // If no printer name is provided, use the default printer
    if printer_name.is_empty() {
        printer_name = "default".to_string();
    }

    Ok((cups_server, printer_name, delay, file_names)) // Return the parsed values
}
