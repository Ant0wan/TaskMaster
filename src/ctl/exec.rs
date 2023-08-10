use std::io::Read;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub fn exec() {
    let mut stream: UnixStream = UnixStream::connect("/path/to/socket.sock").expect("Failed to connect to socket");

    let message: &str = "Hello from Client!";
    stream.write_all(message.as_bytes()).expect("Failed to send message");

    let mut response: String = String::new();
    stream.read_to_string(&mut response).expect("Failed to read response");
    println!("Client Received: {}", response);
}
