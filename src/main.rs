mod defaultp;
mod manual;
mod parser;

use ipp::payload::IppPayload;
use ipp::prelude::*;
use std::fs::File;
use std::io::{Cursor, Read};
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
    fn print_files(&self) -> Result<(), Box<dyn std::error::Error>> {
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
    fn print_file(&self, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Load the file to be printed
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Convert buffer to a readable Cursor
        let payload = IppPayload::new(Cursor::new(buffer));

        // Construct the CUPS server URL (IPP protocol)
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

    /// Method to display the current configuration of the printer and job
    fn display_parameters(&self) {
        println!("Executing print job with the following parameters:");
        println!("CUPS Server: {}", self.cups_server);
        println!("Printer Name: {}", self.printer_name);
        println!("Delay Between Prints: {} seconds", self.delay);
        println!("Files to Print: {:?}", self.file_names);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments and retrieve values for CUPS server, printer, delay, and files
    let (cups_server, mut printer_name, delay, file_names) = parser::parse_arguments()?;

    // If the printer name is "default", query the default printer from the CUPS server
    if printer_name == "default" {
        printer_name = defaultp::get_default_printer(&cups_server)?;
    }

    // Create a CupsPrinter instance with the parsed values
    let printer = CupsPrinter::new(cups_server, printer_name, file_names, delay);

    // Display the parameters being used
    printer.display_parameters();

    // Print the files, handling any errors
    printer.print_files()?;

    Ok(())
}
