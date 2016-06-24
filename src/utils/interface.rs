mod error;
use std::process::exit as sys_exit;
use error::SASState;

pub fn help() {
    println!("help for sas");
}

pub fn exit(state: SASState) {
    if state != SASState::Success {
        println!("{}", state);
        sys_exit(1);
    } else {
        sys_exit(0);
    }
}