// UDP Echo Server Implementation

use std::io;
use std::net::UdpSocket;

/// Starts a UDP echo server on the specified address
///
/// # Arguments
/// * 'addr' - the address to bind to (e.g 127.0.0.1:8080)
///
/// # Returns
/// * 'io::Result<()>' - Ok(()) if server runs successfully, Err otherwise.
///
pub fn run_server(addr: &str) -> io::Result<()> {
    // Step 1: Bind UDP socket to specified address
    // This reserves the port and prepares the socket to receive data
    // The bind() operation can fail if:
    // - The port is already in use
    // - We don't have permission to bind this port
    // - The address format is invalid
    let socket = UdpSocket::bind(addr)?;

    println!("UDP Echo Server listening on {}", addr);
    println!("Waiting for clients...\n");

    // Step 2: Create a buffer to store incoming data
    // UDP packets have a maximum size, 65,507 bytes is the theoretical max
    // We use 1024 bytes as a reasonable buffer for most applications
    let mut buf = [0u8; 1024];

    // Step 3: Main server loop - runs forever
    loop {
        // Step 4: Receive data from any client
        // recv_from() blocks (waits) until data arrives
        // It returns:
        // - The number of bytes received
        // - The source address (who sent the data)
        match socket.recv_from(&mut buf) {
            Ok((num_bytes, src_addr)) => {
                // Successfully received data
                println!("Received {} bytes from {}", num_bytes, src_addr);

                // Step 5: Extract the actual data received
                // We only want the bytes that were actually received, not the whole buffer
                let received_data = &buf[..num_bytes];

                // Step 6: Convert bytes to string for display (if valid UTF-8)
                // from_utf8_lossy() converts bytes to string and replace invalids with <?>
                let message = String::from_utf8_lossy(received_data);
                println!("Message: {}", message);

                // Step 7: Echo the data back to sender
                // sent_to() sends data to a specific address
                // We send back what we actually received
                match socket.send_to(received_data, src_addr) {
                    Ok(send_bytes) => {
                        println!("Echoes {} bytes back to {}\n", send_bytes, src_addr);
                    }
                    Err(e) => {
                        eprintln!("Failed to send data: {}\n", e);
                    }
                }
            }
            Err(e) => {
                // Failed to receive
                eprintln!("Error receiving data: {}", e);
            }
        }
    }
}
