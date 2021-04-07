use std::collections::HashMap;
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

pub struct Clients {
    addrs: HashMap<String, usize>,
    interval: Duration,
    duration: Duration,
    size: usize,
}

pub fn build_clients(addrs: HashMap<String, usize>, interval: u64, duration: u64, size: usize) -> Clients {
    Clients {
        addrs,
        interval: Duration::new(interval, 0),
        duration: Duration::new(duration, 0),
        size: size,
    }
}

fn client_handle_connection(mut stream: TcpStream, barrier: Arc<Barrier>, tx: Sender<Rate>, interval:Duration, duration: Duration, size: usize) {
    let buffer = vec![0u8; size];
    let local = format!("{}:{}", stream.local_addr().unwrap().ip(), stream.local_addr().unwrap().port());
    let peer = format!("{}:{}", stream.peer_addr().unwrap().ip(), stream.peer_addr().unwrap().port());

    barrier.wait();
    let mut total_bytes = 0u64;
    let mut total_elapsed = Duration::new(0, 0);
    let mut interval_bytes = 0u64;
    let mut previous_elapsed = Duration::new(0, 0);
    loop {
        let now = SystemTime::now();
        match stream.write_all(&buffer) {
            Ok(_) => {
                match now.elapsed() {
                    Ok(elapsed) => {
                        total_bytes = total_bytes + size as u64;
                        interval_bytes = interval_bytes + size as u64;
                        total_elapsed = total_elapsed + elapsed;
                        if interval < (total_elapsed - previous_elapsed) {
                            let rate = Rate {
                                local: local.to_string(),
                                peer: peer.to_string(),
                                bytes: interval_bytes,
                                elapsed: total_elapsed - previous_elapsed,
                                threads: 1,
                            };
                            interval_bytes = 0;
                            previous_elapsed = total_elapsed;
                            println!("send rate {}", rate);
                        }
                        if total_elapsed.as_secs_f64() > duration.as_secs_f64() {
                            break;
                        }
                    },
                    Err(e) => {
                        println!("{}:{}: error getting elapsed time {}", peer, local, e);
                        break;
                    }
                }
            },
            Err(_) => {
                break;
            }
        }
    }
    tx.send(Rate {
        local: local,
        peer: peer,
        bytes: total_bytes,
        elapsed: total_elapsed,
        threads: 1,
    }).unwrap();
}

impl Clients {
    pub fn connect(self) -> Result<()> {
        let mut children = vec![];
        let mut nthreads = 0;
        for (_, streams) in self.addrs.iter() {
            nthreads = nthreads + streams;
        }
        let barrier = Arc::new(Barrier::new(nthreads));
        let (tx, rx): (Sender<Rate>, Receiver<Rate>) = mpsc::channel();
        for (addr, streams) in self.addrs.iter() {
            for _ in 0..*streams {
                let b = Arc::clone(&barrier);
                let connector = TcpStream::connect(addr)?;
                let interval = self.interval;
                let duration = self.duration;
                let size = self.size;
                let thread_tx = tx.clone();
                children.push(thread::spawn(move || {
                    client_handle_connection(connector, b, thread_tx, interval, duration, size);
                }));
            }
        }

        for child in children {
            let _ = child.join();
        }

        let mut rates = HashMap::new();
        for _ in 0..nthreads {
            let rate = rx.recv().unwrap();
            println!("{}", rate);
            let mut stored_rate = rates.entry(rate.peer.clone()).or_insert(
                Rate{
                    local: "all".to_string(),
                    peer: rate.peer.clone().to_string(),
                    bytes: 0,
                    elapsed: Duration::new(0, 0),
                    threads: 0,
                }
            );
            stored_rate.bytes = stored_rate.bytes + rate.bytes;
            stored_rate.elapsed = stored_rate.elapsed + rate.elapsed;
            stored_rate.threads = stored_rate.threads + rate.threads;
        }
        let mut total_rate = Rate{
            local: "all".to_string(),
            peer:  "all".to_string(),
            bytes: 0,
            elapsed: Duration::new(0, 0),
            threads: 0,
        };
        for (_, v) in rates {
            println!("{}", v);
            total_rate.bytes = total_rate.bytes + v.bytes;
            total_rate.elapsed = total_rate.elapsed + v.elapsed;
            total_rate.threads = total_rate.threads + v.threads;
        }
        println!("{}", total_rate);

        Ok(())
    }
}
