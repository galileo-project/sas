use std::env::Args;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;
use signal::{verify_signal};
use utils::{help, exit};
use state:: SASState;


fn client(msg: String) {
    let mut buff = vec![0u8; 1024];
    let stream = TcpStream::connect("127.0.0.1:9630");
    match stream {
        Ok(mut st) => {
            let wcount = st.write(&msg.as_bytes()).unwrap();
            let rcount = st.read(&mut buff).unwrap();
            println!("{}", str::from_utf8(&buff[0..rcount]).unwrap());
            buff.clear();
        },
        _          => {
            exit(SASState::ConnErr);
        },
    }
}

pub fn start(args: &mut Args) {
    if args.len() <= 0 {
        help();
    } else {
        match verify_signal(&args.next().unwrap()) {
            Ok(signal)    => client(signal),
            Err(e)        => exit(e),
        }
    }
}