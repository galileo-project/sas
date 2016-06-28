/// verify signal
use state::SASState;


pub fn verify_signal(string: &str) ->Result<String, SASState> {
    match string {
        "reload"   => Ok("reload".to_string()),
        _          => Err(SASState::SigErr),
    }
} 