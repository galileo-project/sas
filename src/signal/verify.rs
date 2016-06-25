/// verify signal
use state::SASState;


pub fn verify_signal(string: &str) ->Result<String, SASState> {
    match string {
        "start"    => Ok("start".to_string()),
        "restart"  => Ok("restart".to_string()),
        "kill"     => Ok("kill".to_string()),
        "reload"   => Ok("reload".to_string()),
        _          => Err(SASState::SigErr),
    }
} 