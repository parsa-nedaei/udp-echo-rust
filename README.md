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
```txt
udp-echo-rust
      |
      |__ src
      |    |
      |    |__ utils
      |    |     |__ server.rs
      |    |     |__ client.rs
      |    |
      |    |__ main.rs
      |    |__ utils.rs
      |
      |__ Cargo.toml
      |__ Cargo.lock
```
### Why this structure ?
The above structure is chosen for following reasons:
1. modularity
2. Reusability
3. Clarity
4. Testing

by creating subdirectories under ```src/``` and adding necessary modules in it, it becomes much more easier to understand to look where for what; if we look, ```utils/``` subdirectory is created as a place for adding modules ```server.rs``` and ```client.rs```. then a ```.rs``` file, with exact name of the subdirector in question, is created under ```src/```, just next to ```main.rs``` (file ```utils.rs```).

## ⚙️ ```Cargo.toml``` Configuration
Since ***Rust***'s standard library (```std```) offers excellent networking support out of the box, the ```Cargo.toml``` is kept minimal as below:
```toml
[package]
name = "udp-echo"
version = "0.1.0"
edition = "2024"

[dependencies]
```





