use crate::channel::{Channel, BoxedRetryStrategy};

#[macro_use]
#[cfg(test)]
extern crate assert_matches;

use std::net::SocketAddr;

pub mod channel;
pub mod requests;
pub mod session;

mod requests_info;
mod error_conversion;
mod format;
mod frame;
mod cursor;
mod mbap;

/// errors that should only occur if there is a logic error in the library
#[derive(Debug)]
pub enum LogicError {
    /// We tried to write
    InsufficientBuffer,
    /// Frame or ADU had a bad size (outgoing)
    BadWriteSize,
    /// Bad cursor seek
    InvalidSeek,
    /// We expected a None to be Some
    NoneError,
    /// Logic error from underlying type that couldn't be converted
    Stdio(std::io::Error)
}

/// errors that occur while parsing a frame off a stream (TCP or serial)
#[derive(Debug)]
pub enum FrameError {
    MBAPLengthZero,
    MBAPLengthTooBig(usize),
    UnknownProtocolId(u16)
}

impl LogicError {
    pub fn from(err: std::io::Error) -> LogicError {
        match err.kind() {
            std::io::ErrorKind::WriteZero => LogicError::InsufficientBuffer,
            std::io::ErrorKind::InvalidInput => LogicError::InvalidSeek,
            _ => LogicError::Stdio(err)
        }
    }
}

#[derive(Debug)]
pub enum Error {
    /// We just bubble up std errors from reading/writing/connecting/etc
    Stdio(std::io::Error),
    /// Logic errors that shouldn't happen
    Logic(LogicError),
    /// Framing errors
    Frame(FrameError),
    /// No connection exists
    NoConnection,
    /// Occurs when a channel is used after close
    ChannelClosed,
}

/// Result type used everywhere in this library
pub type Result<T> = std::result::Result<T, Error>;

pub fn create_client_tcp_channel(addr: SocketAddr, retry: BoxedRetryStrategy) -> Channel {
    Channel::new(addr, retry)
}
