mod client;
mod server;
mod rate;
mod version;

use clap::App;
use clap::Arg;

fn main() {
    let matches = App::new("yabba")
        .version(version::VERSION)
        .about("yet another boring bandwidth analyzer")
        .subcommand(App::new("connect")
            .about("Connect to listeners")
            .arg(Arg::new("addrs")
                .short('a')
                .long("addrs")
                .about("connect address(es)")
                .default_value("localhost:5201"))
            .arg(Arg::new("connections")
                .short('c')
                .long("connections")
                .about("number of parallel connections")
                .default_value("1"))
            .arg(Arg::new("duration")
                .short('d')
                .long("duration")
                .about("run duration in seconds")
                .default_value("10"))
            .arg(Arg::new("size")
                .short('s')
                .long("size")
                .about("buffer size")
                .default_value("131072")))
        .subcommand(App::new("listen")
            .about("Listen for clients")
            .arg(Arg::new("addr")
                .short('a')
                .long("addr")
                .about("bind address")
                .default_value("0.0.0.0:5201"))
            .arg(Arg::new("size")
                .short('s')
                .long("size")
                .about("buffer size")
                .default_value("131072")))
        .subcommand(App::new("version")
            .about("Prints detailed version information"))
        .get_matches();
    
    if let Some(ref matches) = matches.subcommand_matches("listen") {
        let addr = matches.value_of("addr").unwrap();
        let size = matches.value_of("size").unwrap().parse::<usize>().unwrap();
        let s = server::build_server(addr, size);
        match s.listen() {
            Ok(_) => {
                println!("done.");
            },
            Err(val) => {
                println!("{:?}", val);
            }
        }
    } else if let Some(ref matches) = matches.subcommand_matches("connect") {
        let addrs: Vec<&str> = matches.value_of("addrs").unwrap().split(",").collect();
        let connections = matches.value_of("connections").unwrap().parse::<usize>().unwrap();
        let duration = matches.value_of("duration").unwrap().parse::<u64>().unwrap();
        let size = matches.value_of("size").unwrap().parse::<usize>().unwrap();
        let c = client::build_client(addrs, connections, duration, size);
        match c.connect() {
            Ok(_) => {
                println!("done.");
            },
            Err(val) => {
                println!("{:?}", val);
            }
        }
    } else if let Some(ref _matches) = matches.subcommand_matches("version") {
        version::version();
    }
}