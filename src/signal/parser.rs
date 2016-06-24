/// parse signal string to Signal
mod signal;
use self::signal::Signal;

pub fn string_to_signal(signal: String) -> Option<Signal> {
    match signal {
        "start"     => Some(Signal::Start),
        "restart"   => Some(Signal::Restart),
        "kill"      => Some(Signal::Kill),
        "reload"    => Some(Signal::Reload),
        _           => None,
    }
}

pub fn signal_to_string(signal: Signal) -> Option(String) {
    match signal {
        Signal::Start   => Some("start"),
        Signal::Restart => Some("restart"),
        Signal::Kill    => Some("kill"),
        Signal::Reload  => Some("reload"),
        _               => None,
    }
}