use std::{collections::HashMap, io::Read, thread::sleep};

use rust_ib2c_shared_data::SharedData;

fn main() {

    let mut data_map = HashMap::<String, SharedData>::new();
    // Spawn tcp server and connect to port to receive ans print SharedData
    let mut tcp_socket = std::net::TcpStream::connect("127.0.0.1:13337").unwrap();
    let mut last_print = std::time::Instant::now();

    loop {
        let mut buffer = vec![0; 512];
        let Ok(size) = tcp_socket.peek(&mut buffer) else {
            println!("Failed to read from socket");
            continue;
        };
        if size == 0 {
            sleep(std::time::Duration::from_millis(1));
            continue;
        }
        let Ok(size) = tcp_socket.read(&mut buffer) else {
            println!("Failed to read from socket");
            continue;
        };
        let data: Result<SharedData,_> = serde_json::from_slice(&buffer[..size]);
        let Ok(data) = data else {
            sleep(std::time::Duration::from_millis(1));
            continue;
        };
        data_map.entry(data.source.clone()).and_modify(|e| *e = data.clone()).or_insert(data);

        if last_print.elapsed().as_secs_f64() >= 0.2 {
            print!("\x1B[2J\x1B[1;1H");
            print!("--- Current State ---\n");
            for value in data_map.values() {
                print!("{}\n", value.source);
                println!("=> Index: {}, Activity: {:.2}, Target: {:.2}, Stim: {:.2}, Inhib: {:.2}", 
                    value.index, 
                    value.activity, 
                    value.target_rating, 
                    value.stimulation, 
                    value.inhibition,
                );
            }
            last_print = std::time::Instant::now();
        }
    }

}
