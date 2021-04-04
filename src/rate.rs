use std::time::Duration;

pub struct Rate {
    pub bytes: u64,
    pub elapsed: Duration,
}

impl Rate {
    pub fn rate(self) -> f64 {
        self.bytes as f64 * 8.0 / self.elapsed.as_secs_f64()
    }

    pub fn human_rate(self) -> String {
        let r = self.rate();
        let unit = vec!["", "K", "M", "G", "T", "P"];
        for (i, u) in unit.iter().enumerate().rev() {
            let rate = r as u64 / 1024u64.pow(i as u32);
            if rate != 0 {
                return format!("{} {}bits/s", rate as u64, u);
            }
        }
        return format!("{} {}bits/s", r as u64, unit[0]);
    }
}
