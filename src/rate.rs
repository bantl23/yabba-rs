use std::fmt;
use std::time::Duration;

#[derive(Debug)]
pub struct Rate {
    pub local: String,
    pub peer: String,
    pub bytes: u64,
    pub elapsed: Duration,
    pub threads: usize,
}

impl fmt::Display for Rate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = self.bytes as f64 * 8.0 / self.elapsed.as_secs_f64() * self.threads as f64;

        let unit = vec!["", "K", "M", "G", "T", "P"];
        let mut human = format!("{:5.2} {}bits/s", r, unit[0]);
        for (i, u) in unit.iter().enumerate().rev() {
            let rate = r as u64 / 1024u64.pow(i as u32);
            let ratef = r / 1024f64.powf(i as f64);
            if rate != 0 {
                human = format!("{:5.2} {}bits/s", ratef, u);
                break;
            }
        }
        write!(f, "local {}, peer {}, rate {}",
            self.local,
            self.peer,
            human,
        )
    }
}
