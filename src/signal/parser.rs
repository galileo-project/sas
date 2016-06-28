/// parse signal string to Signal
use super::Signal;


pub fn string_to_signal(signal: &str) -> Option<Signal> {
    match signal {
        "reload"    => Some(Signal::Reload),
        _           => None,
    }
}

pub fn signal_to_string(signal: Signal) -> Option<String> {
    match signal {
        Signal::Reload  => Some("reload".to_string()),
    }
}