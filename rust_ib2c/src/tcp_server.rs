use std::{io::Write, net::TcpListener, sync::{Arc, Mutex}, time::Duration};

use rust_ib2c_shared_data::SharedData;


#[derive(Default)]
pub struct Parent {
    pub path: String,
    pub tcp_server: TcpServer,
}

#[derive(Default)]
pub struct TcpServer {
    buffer: Arc<Mutex<Option<SharedData>>>,
}

impl TcpServer {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(None)),
        }
    }

    pub fn send(&self, element: SharedData) {
        if let Ok(mut buf) = self.buffer.try_lock() {
            *buf = Some(element);
        }
    }

    pub fn start(&self) {
        // Spawn TCP server thread here
        let buffer = Arc::clone(&self.buffer);
        std::thread::spawn(move || {
            let tcp_socket = TcpListener::bind("127.0.0.1:13337").unwrap();
            tcp_socket.set_ttl(Duration::from_secs(1).as_secs() as u32).unwrap();
            println!("TCP Server listening on port 13337");
            let Ok((mut connection, _)) = tcp_socket.accept() else {
                println!("Failed to accept connection");
                return;
            };
            println!("Client connected: {:?}", connection);
            loop {
                let data = buffer.lock().unwrap().take();
                match data {
                    Some(data) => {
                        let serialized = serde_json::to_vec(&data).unwrap();
                        let length = (serialized.len() as u32).to_be_bytes();
                        if let Err(e) = connection.write_all(&length) {
                            println!("Connection error: {}", e);
                            println!("Searching for new connection...");
                            connection = tcp_socket.accept().unwrap().0;
                            println!("Client connected: {:?}", connection);
                            continue;
                        }
                        if let Err(e) = connection.write_all(&serialized) {
                            println!("Connection error: {}", e);
                            println!("Searching for new connection...");
                            connection = tcp_socket.accept().unwrap().0;
                            println!("Client connected: {:?}", connection);
                            continue;
                        }
                    }
                    None => {
                        std::thread::sleep(std::time::Duration::from_millis(1));
                    }
                }
            }
        });
    }
}

impl Clone for TcpServer {
    fn clone(&self) -> Self {
        TcpServer {
            buffer: Arc::clone(&self.buffer),
        }
    }
}

