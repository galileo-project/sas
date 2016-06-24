/// parse signal string to Signal
use super::Signal;

pub fn string_to_signal(signal: &str) -> Option<Signal> {
    match signal {
        "start"     => Some(Signal::Start),
        "restart"   => Some(Signal::Restart),
        "kill"      => Some(Signal::Kill),
        "reload"    => Some(Signal::Reload),
        _           => None,
    }
}

pub fn signal_to_string(signal: Signal) -> Option<String> {
    match signal {
        Signal::Start   => Some("start".to_string()),
        Signal::Restart => Some("restart".to_string()),
        Signal::Kill    => Some("kill".to_string()),
        Signal::Reload  => Some("reload".to_string()),
        _               => None,
    }
}