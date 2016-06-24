/// verify signal
mod error;
use error::SASState;

pub fn verify_signal(string: String) Result<String, SASErr> {
    match string {
        "start"    => Ok("start"),
        "restart"  => Ok("restart"),
        "kill"     => Ok("kill"),
        "reload"   => Ok("reload"),
        _          => Err(SASState::SigErr),
    }
} 