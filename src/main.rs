use ipp::client::IppClient;
use ipp::model::IppAttribute;
use ipp::model::Uri;
use ipp::prelude::*;
use ipp::request::IppRequest;
use ipp::request::Operation;
use ipp::response::StatusCode;
use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

/// Struct representing the CUPS printer configuration and the print job details
struct CupsPrinter {
    cups_server: String,     // Address of the CUPS server
    printer_name: String,    // Name of the printer
    file_names: Vec<String>, // List of file names to be printed
    delay: u64,              // Delay between prints (in seconds)
}

impl CupsPrinter {
    /// Constructor for CupsPrinter, initializing with server, printer, files, and delay
    fn new(cups_server: String, printer_name: String, file_names: Vec<String>, delay: u64) -> Self {
        CupsPrinter {
            cups_server,
            printer_name,
            file_names,
            delay,
        }
    }

    /// Method to print all files, with delay between prints if more than one file
    fn print_files(&self) -> Result<(), Box<dyn Error>> {
        let file_count = self.file_names.len(); // Count of the files

        // If no files are provided, print a message and return
        if file_count == 0 {
            println!("There is nothing to print.");
            return Ok(());
        }

        // Loop through each file and print it
        for (index, file_name) in self.file_names.iter().enumerate() {
            self.print_file(file_name)?; // Print the current file

            // If more than one file, apply the delay
            if file_count > 1 && index < file_count - 1 {
                println!("Delaying for {} seconds...", self.delay);
                sleep(Duration::from_secs(self.delay)); // Sleep for the specified delay
            }
        }

        Ok(())
    }

    /// Helper method to print a single file using the IPP protocol and CUPS
    fn print_file(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        // Load the file to be printed
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Construct the CUPS server URL (IPP protocol)
        let uri_string = format!("ipp://{}/printers/{}", self.cups_server, self.printer_name);
        let uri = Uri::try_from(uri_string)?;

        // Create a new IPP client targeting the CUPS server
        let client = IppClient::new(uri);

        // Create a new print job request
        let mut req = IppRequest::new(Operation::PrintJob);
        req.operation().attributes().charset("utf-8");
        req.operation().attributes().natural_language("en");
        req.operation().attributes().uri("printer-uri", uri);
        req.operation()
            .attributes()
            .name("job-name", "Rust Print Job");

        // Attach the file content to the request
        req.set_payload(buffer);

        // Send the request to the CUPS server and check the response
        let response = client.send(req)?;

        // Check if the job was successfully submitted
        if response.header().status_code() == StatusCode::SuccessfulOk {
            println!("Successfully submitted print job for file: {}", file_name);
        } else {
            eprintln!(
                "Failed to submit print job for file: {}. Status: {:?}",
                file_name,
                response.header().status_code()
            );
        }

        Ok(())
    }

    /// Method to display the current configuration of the printer and job
    fn display_parameters(&self) {
        println!("Executing print job with the following parameters:");
        println!("CUPS Server: {}", self.cups_server);
        println!("Printer Name: {}", self.printer_name);
        println!("Delay Between Prints: {} seconds", self.delay);
        println!("Files to Print: {:?}", self.file_names);
    }
}

/// Function to parse command-line arguments and return parsed values for server, printer, delay, and file names
fn parse_arguments() -> Result<(String, String, u64, Vec<String>), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect(); // Collect all arguments passed to the program

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

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments and retrieve values for CUPS server, printer, delay, and files
    let (cups_server, printer_name, delay, file_names) = parse_arguments()?;

    // Create a CupsPrinter instance with the parsed values
    let printer = CupsPrinter::new(cups_server, printer_name, file_names, delay);

    // Display the parameters being used
    printer.display_parameters();

    // Print the files, handling any errors
    printer.print_files()?;

    Ok(())
}
