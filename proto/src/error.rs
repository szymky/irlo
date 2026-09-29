use thiserror::Error;

pub type Result<T, E = ProtocolError> = ::std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("an io error occured")]
    Io(#[source] std::io::Error),

    #[error("invalid message: {}", string)]
    InvalidMessage {
        string: String,

        #[source]
        cause: MessageParseError,
    },
}

impl From<std::io::Error> for ProtocolError {
    fn from(value: std::io::Error) -> Self {
        ProtocolError::Io(value)
    }
}

#[derive(Debug, Error)]
pub enum MessageParseError {
    #[error("message is empty")]
    EmptyMessage,

    #[error("some error occured: {}", message)]
    Other { message: String },
}
