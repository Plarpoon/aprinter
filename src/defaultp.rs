use std::error::Error;
use std::process::Command;

/// Function to query the CUPS server for the default printer using `lpstat`
pub fn get_default_printer(cups_server: &str) -> Result<String, Box<dyn Error>> {
    // Use the `lpstat` command to get the default printer
    let output = Command::new("lpstat")
        .arg("-h")
        .arg(cups_server)
        .arg("-d")
        .output()?;

    // Check if the command was successful
    if !output.status.success() {
        return Err("Failed to execute lpstat command".into());
    }

    // Parse the output to extract the default printer name
    let output_str = String::from_utf8(output.stdout)?;
    if let Some((_, printer_name)) = output_str.trim().split_once(": ") {
        return Ok(printer_name.to_string());
    }

    Err("No default printer found".into())
}
