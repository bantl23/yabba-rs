use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use super::rate::Rate;

pub fn write(rx: Receiver<Rate>) {
    let mut stream_rates = HashMap::new();
    let mut done = false;
    while !done {
        match rx.recv() {
            Ok(rate) => {
                println!("{}", rate);
                let mut stored_stream_rate = stream_rates.entry((rate.local.clone(), rate.peer.clone())).or_insert(
                    Rate{
                        local: rate.local,
                        peer: rate.peer,
                        bytes: 0,
                        elapsed: Duration::new(0, 0),
                        threads: 0,
                    }
                );
                stored_stream_rate.bytes = stored_stream_rate.bytes + rate.bytes;
                stored_stream_rate.elapsed = stored_stream_rate.elapsed + rate.elapsed;
                stored_stream_rate.threads = rate.threads;
            },
            Err(_) => {
                done = true
            }
        }
    }
    let mut server_rates = HashMap::new();
    for (_, rate) in stream_rates {
        let mut stored_server_rate = server_rates.entry(rate.peer.clone()).or_insert(
            Rate{
                local: "all".to_string(),
                peer: rate.peer,
                bytes: 0,
                elapsed: Duration::new(0, 0),
                threads: 0,
            }
        );
        stored_server_rate.bytes = stored_server_rate.bytes + rate.bytes;
        stored_server_rate.elapsed = stored_server_rate.elapsed + rate.elapsed;
        stored_server_rate.threads = stored_server_rate.threads + rate.threads;
    }

    let mut total_rate = Rate{
        local: "all".to_string(),
        peer:  "all".to_string(),
        bytes: 0,
        elapsed: Duration::new(0, 0),
        threads: 0,
    };
    for (_, rate) in server_rates {
        println!("{}", rate);
        total_rate.bytes = total_rate.bytes + rate.bytes;
        total_rate.elapsed = total_rate.elapsed + rate.elapsed;
        total_rate.threads = total_rate.threads + rate.threads;
    }
    println!("{}", total_rate);
}
