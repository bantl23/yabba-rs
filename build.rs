use vergen::Config;
use vergen::vergen;

fn main() {
    // Generate the default 'cargo:' instruction output
    vergen(Config::default()).unwrap()
}