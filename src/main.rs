use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use clap::{arg, command};

fn main() {
    let matches = command!()
        .arg(arg!(-p --port <PORT>).required(false))
        .arg(arg!(-a --address <ADDRESS>).required(false))
        .get_matches();

    let port = match matches.get_one::<u16>("port") {
        Some(p) => p.clone(),
        None => {
            println!("No port specified, using default port (4747).");
            4747
        }
    };

    match matches.get_one::<String>("address") {
        Some(a) => {
            let address = SocketAddr::new(a.parse::<IpAddr>().expect("Error parsing IpAddr"), port);

            println!("Attempting to connect to {}", address.to_string());
            let mut stream = TcpStream::connect(address).expect("Error while establishing connection.");
            println!("Successfully established connection.");
            loop {
                let message = get_input();
                if message == "exit" {
                    break;
                }
                stream.write(message.as_bytes()).expect("Error writing to stream.");
            }
        }
        None => {
            println!("No IP address specified, starting listener on {}", port);
            let listener = TcpListener::bind(std::format!("127.0.0.1:{}", &port)).expect("Error starting listener.");
            for stream in listener.incoming() {
                let mut incoming_message = String::new();
                stream.unwrap().read_to_string(&mut incoming_message).expect("Error reading to string.");
                println!("{}", incoming_message);
            }
        }
    };
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdout().flush().expect("Error flushing output");
    io::stdin().read_line(&mut input).expect("Error reading input.");

    input.trim().to_string()
}