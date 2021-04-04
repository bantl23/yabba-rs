use std::time::Duration;

pub struct Rate {
    pub local: String,
    pub peer: String,
    pub bytes: u64,
    pub elapsed: Duration,
}

impl Rate {
    pub fn rate(self) -> f64 {
        self.bytes as f64 * 8.0 / self.elapsed.as_secs_f64()
    }

    pub fn human_rate(self, threads: usize) -> String {
        let r = self.rate() * threads as f64;
        let unit = vec!["", "K", "M", "G", "T", "P"];
        for (i, u) in unit.iter().enumerate().rev() {
            let rate = r as u64 / 1024u64.pow(i as u32);
            let ratef = r / 1024f64.powf(i as f64);
            if rate != 0 {
                return format!("{:5.2} {}bits/s", ratef, u);
            }
        }
        return format!("{:5.2} {}bits/s", r, unit[0]);
    }
}
