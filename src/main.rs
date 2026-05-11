// main entry
// UDP Echo
// Decides wether to run as server or client based on command-line argument

/// Declare the modules
/// Based on files hierarchy
mod utils;
use utils::{client, server};

/// Declaring libraries to use
/// in 'std' we use 'env' and 'process'
use std::{env, process};

/// Main function
/// Body of code
fn main() {
    // Collect command-line arguments into Vec<String>
    // env::args() returns an iterator over the arguments
    // The first argument (index 0) is always the program name itself
    //
    // Example:
    // ./udp-echo server 127.0.0.1:8080
    // args[0] = "./udp-echo"
    // args[1] = "server"
    // args[2] = "127.0.0.1:8080"
    let args: Vec<String> = env::args().collect();

    // At least 3 arguments are needed: program name, mode, and address
    if args.len() < 3 {
        print_usage(&args[0]);
        process::exit(1); // Exits with code 1 -> indicates to error
    }

    // args[1] is the mode: "server" or "client"
    let mode = &args[1];

    // args[2] is the address: e.g. "127.0.0.1:8080"
    let address = &args[2];

    // Use pattern matching to decide what to run
    // match compares the value against each arm (pattern => action)
    // The underscore _ in the string slice match requires as_str()
    // because match works on values, not references to String
    match mode.as_str() {
        "server" => {
            println!("Starting UDP echo server on address {}...", address);

            // Run server and handle any errors
            // if let Err(e) = ... means: "if result is an Error, bind it to e"
            if let Err(e) = server::run_server(address) {
                eprintln!("Server error: {}", e);
                process::exit(1);
            }
        }

        "client" => {
            println!("Starting UDP client, connecting to {}...", address);

            // Run interactive client and handle any error
            if let Err(e) = client::run_interactive_client(address) {
                eprintln!("Client error: {}", e);
                process::exit(1);
            }
        }

        // The _ pattern is a catch-all - matches anything not matched above
        _ => {
            eprintln!("Unknown mode: {}. (Use 'server' or 'client'.", mode);
            print_usage(&args[0]);
            process::exit(1);
        }
    }
}

/// Prints usage instruction to stderr
///
/// # Arguments
/// * 'program_name' - The name of executable
fn print_usage(program_name: &str) {
    eprintln!("Usage:");
    eprintln!("{} server <address> - Start the echo server", program_name);
    eprintln!("{} client <address> - Start the echo client", program_name);
    eprintln!();
    eprintln!("Examples:");
    eprintln!("{} server 127.0.0.1:8080", program_name);
    eprintln!("{} client 127.0.0.1:8080", program_name);
}
