use std::process::exit as sys_exit;
use state::SASState;


pub fn help() {
    println!("help for sas");
}

pub fn exit(state: SASState) {
    match state {
        SASState::Success => {
            sys_exit(0);
        },
        _                 => {
            println!("{}", state);
            println!("'sas help' for help");
            sys_exit(1);
        }
    };
}