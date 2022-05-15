use calculator_server::Server;
use serde::Deserialize;
use serde_json::Value;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub enum TestElement {
    SendRequest(Value),
    Sleep(Duration),
    ExpectResponse(Value),
    ExpectDisconnect,
}

#[allow(dead_code)]
impl TestElement {
    pub fn send_request(json: &str) -> Self {
        TestElement::SendRequest(serde_json::from_str(json).unwrap())
    }

    pub fn sleep(duration: Duration) -> Self {
        TestElement::Sleep(duration)
    }

    pub fn expect_response(json: &str) -> Self {
        TestElement::ExpectResponse(serde_json::from_str(json).unwrap())
    }

    pub fn expect_disconnect() -> Self {
        TestElement::ExpectDisconnect
    }
}

pub fn single_thread_test(job: Vec<TestElement>, timeout: Duration) {
    multi_thread_test(vec![job], timeout);
}

pub fn multi_thread_test(thread_jobs: Vec<Vec<TestElement>>, timeout: Duration) {
    panic_after(2 * timeout, move || {
        let server = Server::default();
        let port = server.port();
        let server = Arc::new(server);

        let mut thread_handles = Vec::new();
        let server_clone = server.clone();
        thread_handles.push(thread::spawn(move || {
            server_clone.start();
        }));
        for job in thread_jobs {
            let thread = thread::spawn(move || {
                test(port, job);
            });
            thread_handles.push(thread);
            thread::sleep(Duration::from_millis(10));
        }
        thread::sleep(timeout);
        server.stop();
        println!("Waiting for threads to finish");
        for thread in thread_handles {
            thread.join().unwrap();
        }
    });
}

fn panic_after<T, F>(d: Duration, f: F) -> T
where
    T: Send + 'static,
    F: FnOnce() -> T,
    F: Send + 'static,
{
    let (done_tx, done_rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let val = f();
        done_tx.send(()).expect("Unable to send completion signal");
        val
    });

    match done_rx.recv_timeout(d) {
        Ok(_) => handle.join().expect("Thread panicked"),
        Err(_) => panic!("Thread took too long"),
    }
}

fn test(port: u16, elements: Vec<TestElement>) {
    let tcp_stream = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
    let mut deserializer = serde_json::Deserializer::from_reader(&tcp_stream);
    for element in elements {
        match element {
            TestElement::SendRequest(request) => {
                let mut tcp_stream = &tcp_stream;
                serde_json::to_writer(tcp_stream, &request).unwrap();
                tcp_stream.flush().unwrap();
            }
            TestElement::Sleep(duration) => {
                thread::sleep(duration);
            }
            TestElement::ExpectResponse(expected) => {
                let actual = Value::deserialize(&mut deserializer).unwrap();
                assert_eq!(actual, expected);
            }
            TestElement::ExpectDisconnect => {
                let mut tcp_stream = &tcp_stream;
                assert_eq!(tcp_stream.read(&mut [0]).unwrap(), 0);
            }
        }
    }
}
