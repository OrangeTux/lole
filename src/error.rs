/// An error that can occur in this crate.
#[derive(Debug)]
pub struct ParseError {
    kind: ErrorKind,
}

impl ParseError {
    pub fn new(kind: ErrorKind) -> ParseError {
        ParseError { kind }
    }
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
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
            ErrorKind::InvalidTeam(v) => {
                write!(f, "{:?} is not a valid Team", v)
            }
            ErrorKind::InvalidDriver(v) => {
                write!(f, "{:?} is not a valid Driver", v)
            }
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidPenaltyType(u8),
    InvalidPacketType(u8),
    InvalidInfringementType(u8),
    InvalidTeam(u8),
    InvalidDriver(u8),
}

#[derive(Debug)]
pub enum AppError {
    IOError(std::io::Error),
    ParseError(ParseError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::IOError(e) => {
                write!(f, "Failed to read frame: {:?}", e)
            }
            AppError::ParseError(e) => {
                write!(f, "Failed to parse frame: {:?}", e)
            }
        }
    }
}

impl std::convert::From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IOError(e)
    }
}

impl std::convert::From<ParseError> for AppError {
    fn from(e: ParseError) -> Self {
        AppError::ParseError(e)
    }
}

impl std::error::Error for AppError {}
