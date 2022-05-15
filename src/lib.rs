use std::net::TcpListener;
use std::net::TcpStream;

use std::sync::atomic::AtomicBool;

pub struct Server {
    tcp_listener: TcpListener,
    running: AtomicBool,
    // TODO: you can add other fields
}

impl Default for Server {
    fn default() -> Self {
        let tcp_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        Server {
            tcp_listener,
            running: AtomicBool::new(true),
        }
    }
}

impl Server {
    pub fn start(&self) {
        for (id, stream) in self.tcp_listener.incoming().enumerate() {
            if !self.running.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            todo!();
        }
    }

    pub fn stop(&self) {
        // We set the flag and make a new connection to make sure that the server checks the flag
        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);
        TcpStream::connect(("127.0.0.1", self.port())).unwrap();

        // TODO: close all streams using `shutdown`
    }

    pub fn port(&self) -> u16 {
        self.tcp_listener.local_addr().unwrap().port()
    }
}

#[derive(Debug)]
pub enum Command {
    Add(i32),
    Multiply(i32),
    Divide(i32),
    Result,
    History,
    MyHistory,
    Exit,
}

#[derive(Debug)]
pub enum Response {
    Result(i32),
    Ok,
    History(Vec<(u16, Command)>),
    MyHistory(Vec<Command>),
    Error(String),
}
