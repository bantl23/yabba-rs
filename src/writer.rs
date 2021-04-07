use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use super::rate::Rate;

pub fn _write(rx: Receiver<Rate>) {
    let mut rates = HashMap::new();
    let mut done = false;
    while !done {
        match rx.recv() {
            Ok(rate) => {
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
            },
            Err(_) => {
                done = true
            }
        }
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
}
