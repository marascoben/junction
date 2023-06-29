use std::io;
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use std::thread;

pub trait LoadBalancer {
    fn next(&mut self) -> Option<SocketAddr>;
}

pub fn merge_streams(source: TcpStream, destination: TcpStream) {
    let s_arc = Arc::new(source);
    let d_arc = Arc::new(destination);

    let (mut s_tx, mut s_rx) = (s_arc.try_clone().unwrap(), s_arc.try_clone().unwrap());
    let (mut d_tx, mut d_rx) = (d_arc.try_clone().unwrap(), d_arc.try_clone().unwrap());

    let connections = vec![
        thread::spawn(move || io::copy(&mut s_tx, &mut d_rx).unwrap()),
        thread::spawn(move || io::copy(&mut d_tx, &mut s_rx).unwrap()),
    ];

    for t in connections {
        t.join().unwrap();
    }
}
