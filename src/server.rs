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
    size: usize,
}

pub fn build_server(addr: &str, size: usize) -> Server {
    Server {
        addr: addr.to_string(),
        size: size,
    }
}

fn listen_handle_connection(mut stream: TcpStream, size: usize) {
    let mut buffer = vec![0u8; size];
    let mut total_elapsed = Duration::new(0, 0);
    let mut total_bytes = 0u64;
    loop {
        let now = SystemTime::now();
        match stream.read_exact(&mut buffer) {
            Ok(_) => {
                match now.elapsed() {
                    Ok(elapsed) => {
                        total_bytes = total_bytes + size as u64;
                        total_elapsed = total_elapsed + elapsed;
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
    println!("bytes: {}, elapsed: {:?}, rate: {}",
        total_bytes,
        total_elapsed,
        Rate{
            bytes: total_bytes,
            elapsed: total_elapsed,
            threads: 1,
        }.human_rate(),
    );
}

impl Server {
    pub fn listen(self) -> Result<()> {
        let listener = TcpListener::bind(self.addr)?;
        for stream in listener.incoming() {
            let stream = stream?;
            let size = self.size;
            thread::spawn(move || {
                listen_handle_connection(stream, size);
            });
        }
        Ok(())
    }
}
