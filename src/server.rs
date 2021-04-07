use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Result;
use std::time::Duration;
use std::time::SystemTime;
use std::thread;
use super::rate::Rate;

pub struct Server {
    addr: String,
    interval: Duration,
    size: usize,
}

pub fn build_server(addr: &str, interval: u64, size: usize) -> Server {
    Server {
        addr: addr.to_string(),
        interval: Duration::new(interval, 0),
        size: size,
    }
}

fn listen_handle_connection(mut stream: TcpStream, interval: Duration, size: usize) {

    let local = format!("{}:{}", stream.local_addr().unwrap().ip(), stream.local_addr().unwrap().port());
    let peer = format!("{}:{}", stream.peer_addr().unwrap().ip(), stream.peer_addr().unwrap().port());
    let mut buffer = vec![0u8; size];
    let mut total_bytes = 0u64;
    let mut total_elapsed = Duration::new(0, 0);
    let mut interval_bytes = 0u64;
    let mut previous_elapsed = Duration::new(0, 0);

    println!("accepted connection from {}", peer);
    loop {
        let now = SystemTime::now();
        match stream.read_exact(&mut buffer) {
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
                    },
                    Err(e) => {
                        println!("error getting elapsed time {}", e);
                        break;
                    }
                }
            },
            Err(_) => {
                break;
            }
        }
    }
    let rate = Rate{
        local: local,
        peer: peer,
        bytes: total_bytes,
        elapsed: total_elapsed,
        threads: 1,
    };

    println!("{}", rate);
}

impl Server {
    pub fn listen(self) -> Result<()> {
        let listener = TcpListener::bind(self.addr)?;
        for stream in listener.incoming() {
            let stream = stream?;
            let interval = self.interval;
            let size = self.size;
            thread::spawn(move || {
                listen_handle_connection(stream, interval, size);
            });
        }
        Ok(())
    }
}
