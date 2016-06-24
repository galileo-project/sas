use std::fmt;

enum SASState {
    Success,
    Conn,
    Unknown,
    SigErr,
}

impl fmt::Display for SASState {
    fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            SASState::Conn    => "connection error",
            SASState::Unknown => "unknown error",
            SASState::SigErr  => "invalid signal",
            SASState::Success => "success",
        }

        write!(f, "SAS Error: {}", msg);
    }
}