use crate::manual;
use std::env;
use std::error::Error;

/// Function to parse command-line arguments and return parsed values for CUPS server,
/// printer, delay, and file names.
/// It returns a tuple containing (CUPS server, printer name, delay, file names),
/// or an error if parsing fails.
pub fn parse_arguments() -> Result<(String, String, u64, Vec<String>), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect(); // Collect all command-line arguments into a vector

    // If the user provides the `-help` flag, display the manual and exit the program
    if args.contains(&String::from("-help")) {
        manual::print_manual();
        std::process::exit(0);
    }

    // Default values
    let mut cups_server = "localhost".to_string(); // Default to localhost if no URL is provided
    let mut printer_name = None; // Printer name is mandatory, so there's no default
    let mut file_names: Vec<String> = Vec::new(); // Vector to hold the list of file names to print
    let mut delay = 1u64; // Default delay is 1 second between print jobs

    let mut i = 1; // Start parsing arguments from index 1 (skip the program name)
    while i < args.len() {
        match args[i].as_str() {
            "-url" => {
                // Parse the CUPS server address after the `-url` argument
                if i + 1 < args.len() {
                    cups_server = args[i + 1].clone(); // Store the provided CUPS server URL
                    i += 2; // Move to the next argument
                } else {
                    return Err("Expected argument after -url".into()); // Return error if no URL is provided
                }
            }
            "-pname" => {
                // Parse the printer name after the `-pname` argument
                if i + 1 < args.len() {
                    printer_name = Some(args[i + 1].clone()); // Store the provided printer name
                    i += 2; // Move to the next argument
                } else {
                    return Err("Expected argument after -pname".into()); // Return error if no printer name is provided
                }
            }
            "-delay" => {
                // Parse the delay value after the `-delay` argument
                if i + 1 < args.len() {
                    let delay_str = &args[i + 1]; // Retrieve the delay value as a string
                    delay = delay_str
                        .parse::<u64>()
                        .map_err(|_| "Delay must be a valid integer")?; // Parse the delay value as a u64 integer
                    i += 2; // Move to the next argument
                } else {
                    return Err("Expected integer argument after -delay".into());
                    // Return error if no delay value is provided
                }
            }
            "-files" => {
                // Parse the file names after the `-files` argument
                if i + 1 < args.len() {
                    file_names = args[i + 1..].to_vec(); // Collect all remaining arguments as file names
                    break; // Stop parsing as files are always the last arguments
                } else {
                    return Err("Expected argument after -files".into()); // Return error if no file names are provided
                }
            }
            _ => {
                // Return an error for any unknown or unexpected argument
                return Err(format!("Unknown argument: {}", args[i]).into());
            }
        }
    }

    // Ensure that a printer name is provided (mandatory)
    let printer_name = printer_name.ok_or("Printer name (-pname) is required")?;

    // Return the parsed values (CUPS server, printer name, delay, and file names)
    Ok((cups_server, printer_name, delay, file_names))
}
