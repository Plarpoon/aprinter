mod manual;
mod parser;
mod version;

use ipp::payload::IppPayload;
use ipp::prelude::*;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

/// Struct representing the CUPS printer configuration and the print job details
struct CupsPrinter {
    cups_server: String,     // The address of the CUPS server
    printer_name: String,    // The name of the printer
    file_names: Vec<String>, // List of files to be printed
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
    fn print_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_count = self.file_names.len(); // Count the number of files to be printed

        // If no files are provided, print a message and return early
        if file_count == 0 {
            println!("There is nothing to print.");
            return Ok(());
        }

        let mut valid_files = vec![]; // A list to store the valid (existing) files

        // Check if each file exists and collect valid files
        for file_name in &self.file_names {
            if Path::new(file_name).exists() {
                valid_files.push(file_name.clone());
            } else {
                eprintln!("Warning: File does not exist: {}", file_name); // Print warning for missing file
            }
        }

        let valid_file_count = valid_files.len();

        // If no valid files are found, print a message and return early
        if valid_file_count == 0 {
            println!("No valid files to print.");
            return Ok(());
        }

        // Loop through each valid file and print it
        for (index, file_name) in valid_files.iter().enumerate() {
            self.print_file(file_name)?; // Print the current file

            // If more than one file, apply the delay between prints
            if valid_file_count > 1 && index < valid_file_count - 1 {
                println!("Delaying for {} seconds...", self.delay);
                sleep(Duration::from_secs(self.delay)); // Sleep for the specified delay
            }
        }

        Ok(())
    }

    /// Helper method to print a single file using the IPP protocol and CUPS
    fn print_file(&self, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Open the file to be printed
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new(); // Create a buffer to store the file content
        file.read_to_end(&mut buffer)?; // Read the file content into the buffer

        // Convert buffer to a readable Cursor to be used as the print payload
        let payload = IppPayload::new(Cursor::new(buffer));

        // Construct the CUPS server URI (IPP protocol)
        let uri: Uri =
            format!("ipp://{}/printers/{}", self.cups_server, self.printer_name).parse()?;

        // Create a new IPP client targeting the CUPS server
        let client = IppClient::new(uri.clone());

        // Create a new print job operation with the payload
        let operation = IppOperationBuilder::print_job(uri, payload).build();

        // Send the request to the CUPS server and check the response
        let response = client.send(operation)?;

        // Check if the job was successfully submitted
        if response.header().status_code().is_success() {
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

    /// Method to display the current configuration
    fn display_parameters(&self) {
        println!("Executing print job with the following parameters:");
        println!("CUPS Server: {}", self.cups_server); // Display the CUPS server address
        println!("Printer Name: {}", self.printer_name); // Display the printer name
        println!("Delay Between Prints: {} seconds", self.delay); // Display the delay
        println!("Files to Print: {:?}", self.file_names); // Display the list of files to be printed
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments and retrieve values for CUPS server, printer, delay, and files
    let (cups_server, printer_name, delay, file_names) = parser::parse_arguments()?;

    // Create a CupsPrinter instance with the parsed values
    let printer = CupsPrinter::new(cups_server, printer_name, file_names, delay);

    // Display the parameters being used for the print job
    printer.display_parameters();

    // Print the files, handling any errors that occur
    printer.print_files()?;

    Ok(())
}
