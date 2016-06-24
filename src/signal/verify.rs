/// verify signal
use state::SASState;

pub fn verify_signal(string: String) ->Result<String, SASState> {
    match string {
        "start"    => Ok("start"),
        "restart"  => Ok("restart"),
        "kill"     => Ok("kill"),
        "reload"   => Ok("reload"),
        _          => Err(SASState::SigErr),
    }
} 