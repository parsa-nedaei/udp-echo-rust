// UDP Echo client code

use std::io;
use std::net::UdpSocket;
use std::time::Duration;

/// Sends a message to a UDP echo server and receives the response
///
/// # Argument
/// * 'server_addr' - The server's address (e.g 127.0.0.1:8080)
/// * 'message' - The message to send to server
///
///
/// # Returns
/// * 'io::Result<String>' - The echoed message from the server, or an error if failed
pub fn send_message(server_addr: &str, message: &str) -> io::Result<String> {
    // Step 1: Create a UDP socket
    // We bind to "0.0.0.0:0" which means:
    // - 0.0.0.0: bind to all available network interfaces
    // - :0: let the OS choose an available port automatically
    // This is typical for clients - we don't care which port we use
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Step 2: Set a timeout for receiving data
    // Without a timeout, recv_from() would block forever if the server doesn't respond
    // Duration::from_secs(5) creates a 5-second timeout
    // set_read_timeout() configures how long recv_from() will wait
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;

    println!("Client bound to local address: {}", socket.local_addr()?);
    println!("Sending message to server at {}", server_addr);

    // Step 3: Convert the message string into bytes
    // Network communication works with bytes, not string
    // as_bytes() converts '&str' to '&[u8]' (byte slice)
    let message_bytes = message.as_bytes();

    // Step 4: Send the message to the server
    // send_to() sends data to a specific address
    // it returns the number of bytes actually sent
    let send_bytes = socket.send_to(message_bytes, server_addr)?;
    println!("Send {} bytes: \"{}\"", send_bytes, message);

    // Step 5: Prepare a buffer to receive the response
    // Same as server - 1024 is sufficient for most messages
    let mut buf = [0u8; 1024];

    // Step 6: Receive the echoed response from the server
    // recv_from() blocks until data arrives or timeout occurs
    // it returns number of bytes received and sender's address
    let (num_bytes, src_addr) = socket.recv_from(&mut buf)?;

    println!("Received {} bytes from {}", num_bytes, src_addr);

    // Step 7: Converts the received bytes back to string
    // we use from_utf8_lossy() which handles invalid UTF-8 gracefully (replace by <?>)
    let response = String::from_utf8_lossy(&buf[..num_bytes]);

    // Step 8: Returns the response as an owned String
    // to_string() converts the Cow<str> from from_utf8_lossy() to string
    Ok(response.to_string())
}

/// Runs an interactive UDP client that allows multiple messages
///
/// # Arguments
/// * 'server_addr' - The servers address to connect to
///
/// # Returns
/// * 'io::Result<()>' - Ok(()) if client runs successfully, Err if fails
///
pub fn run_interactive_client(server_addr: &str) -> io::Result<()> {
    println!("=== UDP Echo Client ===");
    println!("Server address: {}", server_addr);
    println!("Type your messages (or 'quit' to exit)\n");

    // Import stdin for reading user input
    use std::io::{BufRead, stdin};

    // Create a buffered reader for standard input
    // stdin() returns a handle to the standard input stream
    // lock() acquires a lock on stdin for efficient reading
    let stdin = stdin();
    let reader = stdin.lock();

    // Iterate over each line of input
    // lines() returns an iterator over the lines of input
    for line in reader.lines() {
        // line is a Result<String, Error>
        // The ? operator unwraps it or returns the error
        let input = line?;

        // Trim whitespace from both ends
        let trimmed = input.trim();

        // Check if user wants to quit
        if trimmed.eq_ignore_ascii_case("quit") {
            println!("Goodbye...");
            break;
        }

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Send the message and handle the result
        match send_message(server_addr, trimmed) {
            Ok(response) => {
                println!("Echo from server: \"{}\"\n", response);
            }
            Err(e) => {
                eprintln!("Error: {}\n", e);
            }
        }
    }
    Ok(())
}
