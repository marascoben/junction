mod balancer;
mod roundrobin;

use balancer::{LoadBalancer, merge_streams};
use roundrobin::RoundRobin;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8081")?;

    let mut load_balancer = RoundRobin::new(vec![
        "127.0.0.1:5000".to_string(),
        "127.0.0.1:5001".to_string(),
        "127.0.0.1:5002".to_string(),
        "127.0.0.1:5003".to_string(),
    ]);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let connector = TcpStream::connect(load_balancer.next().unwrap())
                    .map(|server| thread::spawn(move || merge_streams(stream, server)));

                match connector {
                    Ok(_) => continue,
                    Err(_) => continue,
                }
            }
            Err(_) => continue,
        }
    }
    Ok(())
}
