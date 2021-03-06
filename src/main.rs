mod clients;
mod server;
mod rate;
mod version;
mod writer;

use std::collections::HashMap;
use clap::Arg;
use clap::Command;

fn main() {
    let matches = Command::new("yabba")
        .version(env!("VERGEN_GIT_SEMVER"))
        .about("yet another boring bandwidth analyzer")
        .arg_required_else_help(true)
        .subcommand(Command::new("connect")
            .about("Connect to listeners")
            .arg(Arg::new("addrs")
                .short('a')
                .long("addrs")
                .help("connect address(es) with stream counts")
                .default_value("localhost:5201#1"))
            .arg(Arg::new("interval")
                .short('i')
                .long("interval")
                .help("report interval in seconds")
                .default_value("2"))
            .arg(Arg::new("duration")
                .short('d')
                .long("duration")
                .help("run duration in seconds")
                .default_value("10"))
            .arg(Arg::new("size")
                .short('s')
                .long("size")
                .help("buffer size")
                .default_value("131072")))
        .subcommand(Command::new("listen")
            .about("Listen for clients")
            .arg(Arg::new("addr")
                .short('a')
                .long("addr")
                .help("bind address")
                .default_value("0.0.0.0:5201"))
            .arg(Arg::new("interval")
                .short('i')
                .long("interval")
                .help("report interval in seconds")
                .default_value("2"))
            .arg(Arg::new("size")
                .short('s')
                .long("size")
                .help("buffer size")
                .default_value("131072")))
        .subcommand(Command::new("version")
            .about("Prints detailed version information"))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("listen") {
        let addr = matches.value_of("addr").unwrap();
        let interval = matches.value_of("interval").unwrap().parse::<u64>().unwrap();
        let size = matches.value_of("size").unwrap().parse::<usize>().unwrap();
        let s = server::build_server(addr, interval, size);
        match s.listen() {
            Ok(_) => {},
            Err(val) => {
                println!("{:?}", val);
            }
        }
    } else if let Some(ref matches) = matches.subcommand_matches("connect") {
        let mut addrs: HashMap<String, usize> = HashMap::new();
        for i in matches.value_of("addrs").unwrap().split(",") {
            let j: Vec<&str> = i.split("#").collect(); 
            let addr = j[0];
            let streams = j[1].parse::<usize>().unwrap();
            addrs.insert(addr.to_string(), streams);
        }
        let interval = matches.value_of("interval").unwrap().parse::<u64>().unwrap();
        let duration = matches.value_of("duration").unwrap().parse::<u64>().unwrap();
        let size = matches.value_of("size").unwrap().parse::<usize>().unwrap();
        let c = clients::build_clients(addrs, interval, duration, size);
        match c.connect() {
            Ok(_) => {},
            Err(val) => {
                println!("{:?}", val);
            }
        }
    } else if let Some(ref _matches) = matches.subcommand_matches("version") {
        version::version();
    }
}