use std::env::Args;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::thread;
use std::str;
use signal::{Signal, string_to_signal};


fn handle_signal(sig_str: &str) {
    let signal = string_to_signal(sig_str);

    match signal {
        Some(Signal::Start)     => println!("strat"),
        Some(Signal::Restart)   => println!("strat"),
        Some(Signal::Reload)    => println!("reload"),
        Some(Signal::Kill)      => println!("kill"),
        None                    => println!("unknown signal"),
    }
}

fn handle_client(stream: &mut TcpStream) {
    let mut buff = vec![0u8; 1024];
    let rcount = stream.read(&mut buff).unwrap();
    handle_signal(str::from_utf8(&buff[0..rcount]).unwrap());
}

fn server() {
    let listener = TcpListener::bind("127.0.0.1:9630").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut st) => {
                thread::spawn(move||{
                    handle_client(&mut st);
                });
            },
            Err(e) => {
                println!("Error: {}", e);
            },
        }
    }

    drop(listener);
}

pub fn start(args: &mut Args) {
    println!("start server ...");
    server();
}