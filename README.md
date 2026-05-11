# 🦀 UDP Echo Server and Client
Provided codes are UDP echo server and client, written in rust. It can be used for understanding the concept of UDP protocol, via two terminals, one as server and other as client. Despite TCP, UDP does not require prior communication (handshake) to establish channel or data path. Within the codes, apropriate comments are placed for better understanding of operation of each part. Before discussion in more details, for a breif overview, dear user should run both server and client with apropriate arguments containing the mode (server or client) and address (say 127.0.0.1:8080), in two seperated terminals. Once have done, send arbitrary messages from client terminal and monitor the response (echo) of server on the other.

## 📝 Introduction to UDP and its Concepts
### What's UDP ?
UDP (User Datagram Protocol) is one of the core protocols for communication. Unlike TCP (Transfer Control Protocol), UDP is a connectionless protocol, which means no prior communication of both sides (handshake) is necessary to establish receive and transmission of data. This can be considered as:
- **No connection establishment:** There's no handshake process (like TCP's three-way handshake)
- **No guaranteed delivery:** Packets (called *Datagrams*) may be lost, duplicated, or arrive out of order
- **No flow control:** The sender does not wait for acknowledgments
- **Faster and lighter:** Less overhead means lower latency
