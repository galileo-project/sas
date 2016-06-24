mod server;
mod client;
mod utils;
mod state;
mod signal;

use std::env;
use utils::{help, exit};
use state::SASState;

fn dispatch (option: String, args: &mut env::Args) {
    if option == "daemon" { // server side
        server::start(args);
    } else if option == "-s" {
        client::start(args);
    } else {                // client side
        help();
    }
}

fn main() {
    let mut args = env::args();
    args.next();            // skip program name

    match args.next() {
        Some(option) => dispatch(option, &mut args),
        None         => exit(SASState::CmdErr),
    }
}
