use std::net::{UdpSocket, SocketAddr};
use std::time::{Duration};
use std::thread;

fn send_message(socket: UdpSocket, addr: SocketAddr) {
    loop {
       let message = "Hello, UDP!";
       socket.send_to(message.as_bytes(), addr).expect("Failed to send message");
       thread::sleep(Duration::from_millis(1000)); //1sec
    }
}

fn receive_messages(socket: UdpSocket) {
    let mut buffer = [0u8; 1024];
    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, addr)) => {
                let message = String::from_utf8_lossy(&buffer[..size]);
                println!("Received message from {}: {}", addr, message);
            }
            Err(e) => {
                eprintln!("Failed to receive message: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:54321").expect("Failed to bind socket");

    let send_addr: SocketAddr = "127.0.0.1:54321".parse().expect("Invalid send address");

    let send_socket = socket.try_clone().expect("Failed to clone socket");
    let receive_socket = socket.try_clone().expect("Failed to clone socket");

    let send_thread = thread::spawn(move || {
        send_message(send_socket, send_addr);
    });

    let receive_thread = thread::spawn(move || {
        receive_messages(receive_socket);
    });

    send_thread.join().expect("Send thread panicked");
    receive_thread.join().expect("Receive thread panicked");
}

