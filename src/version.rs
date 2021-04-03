pub static VERSION: &'static str = "v0.9.0";
pub static HASH: &'static str = "unknown";

pub fn version() {
    println!("version {} ({})", VERSION, HASH);
}