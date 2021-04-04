use std::net::TcpStream;
use std::io::Result;
use std::io::Write;
use std::sync::Arc;
use std::sync::Barrier;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::time::SystemTime;
use super::rate::Rate;


use std::thread;

pub struct Client {
    addrs: Vec<String>,
    connections: usize,
    duration: Duration,
    size: usize,
}

pub fn build_client(addrs: Vec<&str>, connections: usize, duration: u64, size: usize) -> Client {
    let a = addrs.iter().map(|&a| a.to_string()).collect::<Vec<String>>();
    Client {
        connections,
        addrs: a,
        duration: Duration::new(duration, 0),
        size: size,
    }
}

fn client_handle_connection(mut stream: TcpStream, barrier: Arc<Barrier>, tx: Sender<Rate>, index: usize, duration: Duration, size: usize) {
    let buffer = vec![0u8; size];
    barrier.wait();
    println!("{} {:?}", index, SystemTime::now());
    let mut total_bytes = 0u64;
    let mut total_elapsed = Duration::new(0, 0);
    loop {
        let now = SystemTime::now();
        match stream.write_all(&buffer) {
            Ok(_) => {
                match now.elapsed() {
                    Ok(elapsed) => {
                        total_bytes = total_bytes + size as u64;
                        total_elapsed = total_elapsed + elapsed;
                        if total_elapsed.as_secs_f64() > duration.as_secs_f64() {
                            break;
                        }
                    },
                    Err(e) => {
                        println!("{}: error getting elapsed time {}", index, e);
                        break;
                    }
                }
            },
            Err(_) => {
                break;
            }
        }
    }
    println!("{}: bytes: {}, elapsed: {:?}, rate: {}",
        index,
        total_bytes,
        total_elapsed,
        Rate{
            bytes: total_bytes,
            elapsed: total_elapsed,
        }.human_rate(),
    );
    tx.send(Rate {
        bytes: total_bytes,
        elapsed: total_elapsed,
    }).unwrap();
}

impl Client {
    pub fn connect(self) -> Result<()> {
        let mut children = vec![];
        let nthreads = self.addrs.len() * self.connections;
        let barrier = Arc::new(Barrier::new(nthreads));
        let (tx, rx): (Sender<Rate>, Receiver<Rate>) = mpsc::channel();
        for addr in self.addrs.iter() {
            for index in 0..self.connections {
                let b = Arc::clone(&barrier);
                let connector = TcpStream::connect(addr)?;
                let duration = self.duration;
                let size = self.size;
                let thread_tx = tx.clone();
                children.push(thread::spawn(move || {
                    client_handle_connection(connector, b, thread_tx, index, duration, size);
                }));
            }
        }

        for child in children {
            let _ = child.join();
        }

        let mut total_bytes = 0u64;
        let mut total_elapsed = Duration::new(0, 0);
        for _ in 0..nthreads {
            let rate = rx.recv().unwrap();
            total_bytes = total_bytes + rate.bytes;
            total_elapsed = total_elapsed + rate.elapsed;
        }
        let total_rate = Rate{
            bytes: total_bytes,
            elapsed: total_elapsed,
        };
        
        println!("total_bytes: {}, total_elapsed {:?}, total rate: {}", total_bytes, total_elapsed, total_rate.human_rate());

        Ok(())
    }
}
