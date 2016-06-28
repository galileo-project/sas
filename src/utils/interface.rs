use std::process::exit as sys_exit;
use state::SASState;


pub fn help() {
    println!("Help for SAS
    sas daemon      Run sas server
    sas -s [signal] Sent signal to sas server
    
    SAS Signals:
    start:    Start service
    stop:     Stop service
    restart:  Restart service
    reload:   Reload service");

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