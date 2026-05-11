# 🦀 UDP Echo Server and Client
Provided codes are UDP echo server and client, written in rust. It can be used for understanding the concept of UDP protocol, via two terminals, one as server and other as client. Despite TCP, UDP does not require prior communication (handshake) to establish channel or data path. Within the codes, apropriate comments are placed for better understanding of operation of each part. Before discussion in more details, for a breif overview, dear user should run both server and client with apropriate arguments containing the mode (server or client) and address (say 127.0.0.1:8080), in two seperated terminals. Once have done, send arbitrary messages from client terminal and monitor the response (echo) of server on the other.

## 📝 Introduction to UDP and its Concepts
### What's UDP ?
UDP (User Datagram Protocol) is one of the core protocols for communication. Unlike TCP (Transfer Control Protocol), UDP is a connectionless protocol, which means no prior communication of both sides (handshake) is necessary to establish receive and transmission of data. This can be considered as:
- **No connection establishment:** There's no handshake process (like TCP's three-way handshake)
- **No guaranteed delivery:** Packets (called *Datagrams*) may be lost, duplicated, or arrive out of order
- **No flow control:** The sender does not wait for acknowledgments
- **Faster and lighter:** Less overhead means lower latency

### When to use UDP?
This protocol is an ideal choice for applications where:
1. **Speed is more important then reliability:** Real-time gaming, live video streaming
2. **Small, independent messages:** DNS queries, DHCP
3. **Broadcast/Multicast:** Sending to multiple recipients simultaneously
4. **Loss tolerance:** VoIP (Voice over IP), where occasional packet loss is acceptable

### Real-World Applications
Some examples for these applications are:
- **DNS (Domain Name System):** Quick lookups without connection overhead
- **Vidoe Streaming:** Netflix, Youtube (with application-level error handling)
- **Online Gaming:** Fast-paced games where old date becomes irrelevant quickly
- **IoT (Internet of Things) Sensors:** Lightweight data transmission from sensors
- **VoIP (Voice over IP):** Skype, Zoom audio/video calls

## 🏗️ Project Hierarchy
The structure of files are as below:
```rawtxt
[udp-echo-rust]
      |
      |__ src/
      |    |
      |    |__ utils/
      |    |     |
      |    |     |__ server.rs
      |    |     |
      |    |     |__ client.rs
      |    |
      |    |__ main.rs
      |    |
      |    |__ utils.rs
      |
      |__ Cargo.toml
```
### Why this structure ?
The above structure is chosen for following reasons:
1. modularity
2. Reusability
3. Clarity
4. Testing

by creating subdirectories under ```src/``` and adding necessary modules in it, it becomes much more easier to understand to look where, for what; if we look, ```utils/``` subdirectory is created as a place for adding modules ```server.rs``` and ```client.rs```. then a ```.rs``` file, with exact name of the subdirectory in question, is created under ```src/```, just next to ```main.rs``` (file ```utils.rs```).

## ⚙️ ```Cargo.toml``` Configuration
Since ***Rust***'s standard library (```std```) offers excellent networking support out of the box, the [```Cargo.toml```](https://github.com/parsa-nedaei/udp-echo-rust/blob/main/Cargo.toml) is kept minimal as below:
```toml
[package]
name = "udp-echo-rust"
version = "0.1.0"
edition = "2024"

[dependencies]
```
Therefore, no external crates are necessary for basic UDP communication, and we use ``std::net::UdpSocket``.
### Why no dependencies ?
- Rust's ```std::net``` module provides robust UDP support
- Keeps the porject lightweight and compilation fast
- Demonstrates Rust's powerful standard library

## 🖥️ UDP Server Implemntation
The UDP server (implemented in [```server.rs```](https://github.com/parsa-nedaei/udp-echo-rust/blob/main/src/utils/server.rs)) needs to do:
1. **Bind to an address:** Reserve a specific IP addresss and port to listen on
2. **Receive data:** Wait for incoming UDP packets (Datagrams)
3. **Process data:** In our case, it's a simple echoing back
4. **Send response:** Send the data back to the sender's address

### Key Rust Concepts used
#### ```std::net::UdpSocket```
The main type for UDP communication in Rust. It represents a UDP socket bound to a local address. For instance, the following structure is used to bind address via ```UdpSocket```:
```rust
let socket = match UdpSocket::bind("127.0.0.1:8080") {
    Ok(s) => s,
    Err(e) => return Err(e),
};
```
or for simplicity, we can use ```?``` oprator:
```rust
let socket = UdpSocket::bind("127.0.0.1:8080")?;
```
> **Note:**
>
> for more detail, checkout [```server.rs```](https://github.com/parsa-nedaei/udp-echo-rust/blob/main/src/utils/server.rs).

## 📞 UDP Client Implementation
It is simpler than a server. It needs to:
1. **Create a socket:** Unlike the server, the client doesn’t need to bind to a specific port (the OS assigns one automatically)
2. **Send data:** Send a message to the server’s address
3. **Receive response:** Wait for the echo response from the server
4. **Display result:** Show what was received
> **Note:**
>
> for more detail, checkout [```client.rs```](https://github.com/parsa-nedaei/udp-echo-rust/blob/main/src/utils/client.rs)

## 🏚️ Main Entry
Within ```main.rs``` all parts above, are put together, which makes an integration. This is where the program parses command-line arguments, and decides whether to run as server or client.
For this, we used ```std::env::args()``` in order to read command-line argument.

> **Note:**
>
> For more details, checkout [```main.rs```](https://github.com/parsa-nedaei/udp-echo-rust/blob/main/src/main.rs)

## 👷 How to Use
### Cloning and Building
Start by creating and arbitrary directory:
```shell
mkdir udp_echo
cd udp_echo
```
Then clone:
```shell
git clone https://github.com/parsa-nedaei/udp-echo-rust.git
```
Navigate to ```udp-echo-rust```:
```shell
cd udp-echo-rust
```
Build:
```shell
cargo build --release
```
### Running Project
Now two seperated terminals must be opened, both within the root directory of project. It is necessary to use one as ```server``` and another as ```client```.
#### ```Server``` Terminal
Within server terminal (Terminal 1), run below:
```shell
cargo run --release -q -- server 127.0.0.1:8080
```
entered address could be any available address and port, e.g ```127.0.0.1:10808```. Upon running the above command, you should see:
```txt
Starting UDP echo server on address 127.0.0.1:8080...
UDP Echo Server listening on 127.0.0.1:8080
Waiting for clients...
```
which indicate to successful execution. Now we proceed to ```client```.
#### ```Client``` Terminal
Within client terminal (Terminal 2), run below:
```shell
cargo run --release -q -- client 127.0.0.1:8080
```
> **Note:** Address of client must be the address entered for server !

You should see following:
```shell
Starting UDP client, connecting to 127.0.0.1:8080...
=== UDP Echo Client ===
Server address: 127.0.0.1:8080
Type your messages (or 'quit' to exit)
```
this means that client is executed successfuly and waiting for your message.

> **Note:** You can open another terminal(s) as client, UDP servers can handle multiple clients simultaneously without any special code.

## 🧪 Testing Edge Cases and Understanding UDP Behavior
One can explore UDP’s characteristics by testing edge cases. This will reveal important differences between UDP and TCP, and help you understand when to use UDP. The possible test cases for this project could be as follows:
1. **Packet Loss Simulation**
2. **Out-of-Order Delivery**
3. **Maximum Packet Size**
4. **Empty messages**
5. **Special Characters and Unicode**
6. **Rapid Fire Messages**
Each of above, could give good understanding about how UDP works.

## 🖋️ Final Words
This project is a good start point for the ones looking for network programming with Rust. It is also a good example understand the basics of UDP. Long stroy short, this project is more educational, rather than industrial! Of course, industrial projects can also be written in rust language.
