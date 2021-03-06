use std::env::Args;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::thread;
use std::str;
use signal::{Signal, string_to_signal};


fn handle_signal(sig_str: &str) {
    match string_to_signal(sig_str) {
        Some(Signal::Reload)    => println!("reload"),
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
    println!("Starting sas daemon ...");
    server();
}