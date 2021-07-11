/// An error that can occur in this crate.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidPenaltyType(u8),
    InvalidPacketType(u8),
    InvalidInfringementType(u8),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::InvalidPenaltyType(v) => {
                write!(f, "{:?} is not a valid PenaltyType", v)
            }
            ErrorKind::InvalidPacketType(v) => {
                write!(f, "{:?} is not a valid PacketType", v)
            }
            ErrorKind::InvalidInfringementType(v) => {
                write!(f, "{:?} is not a valid InfringementType", v)
            }
        }
    }
}
