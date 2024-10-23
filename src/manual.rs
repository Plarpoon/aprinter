pub const MANUAL: &str = r#"
NAME
    trak-aprinter - A command-line tool for submitting print jobs to a remote CUPS server via IPP.

SYNOPSIS
    trak-aprinter [-url server] [-pname printer] [-delay delay] [-files files...]

DESCRIPTION
    trak-aprinter is a command-line tool designed to submit print jobs to a remote CUPS server via
    the IPP protocol. It allows users to specify the CUPS server URL, printer name, and list of
    files to be printed, with an optional delay between each print job.

OPTIONS
    -url server
        Specifies the URL or IP address of the CUPS server. Defaults to "localhost" if not provided.

    -pname printer
        Specifies the name of the printer. Defaults to "default" if not provided.

    -delay delay
        Specifies the delay (in seconds) between each print job. Defaults to 1 second.

    -files files...
        Specifies one or more files to be printed. The program will print each file in the order
        provided. If no files are provided, the program will output "There is nothing to print."

    -help
        Displays this help manual.

EXAMPLES
    To print a single file to a printer on a remote server with a 5-second delay:
        trak-aprinter -url "192.168.3.18" -pname "TA_UTAX_2507ci_" -delay 5 -files "file1.txt"

    To print multiple files to a printer on localhost with a 1-second delay (default):
        trak-aprinter -files "file1.txt" "file2.pdf"

EXIT STATUS
    The program exits with status 0 on success and prints any errors encountered during execution.
"#;

pub fn print_manual() {
    println!("{}", MANUAL);
}
