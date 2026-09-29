use thiserror::Error;

pub type Result<T, E = ProtocolError> = ::std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ProtocolError {}
