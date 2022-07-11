//! TTY related functionality.
use crate::config::{PtyConfig};
use crate::event::{OnResize, WindowSize};
use crate::tty::{ChildEvent, EventedPty, EventedReadWrite};
use mio::net::TcpStream;

use std::io::*;

pub struct Pty {
    socket : TcpStream,
    token: mio::Token,
}

impl Pty {
}

/// Create a new localsocket and return a handle to interact with it.
pub fn new(_config: &PtyConfig, _window_size: WindowSize, _window_id: Option<usize>) -> Result<Pty> {

    let s = TcpStream::connect(&"127.0.0.1:8000".parse().unwrap()).unwrap();

    let pty = Pty {socket : s , token : mio::Token::from(1024)};
    Ok(pty)
}

impl EventedReadWrite for Pty {
    type Reader = TcpStream;
    type Writer = TcpStream;

    #[inline]
    fn register(
        &mut self,
        poll: &mio::Poll,
        token: &mut dyn Iterator<Item = mio::Token>,
        _interest: mio::Ready,
        _poll_opts: mio::PollOpt,
    ) -> Result<()> {
        self.token = token.next().unwrap();
        poll.register(
            &self.socket,
            self.token,
            mio::Ready::readable(),
            mio::PollOpt::level(),
        )
    }

    #[inline]
    fn reregister(
        &mut self,
        poll: &mio::Poll,
        _interest: mio::Ready,
        _poll_opts: mio::PollOpt,
    ) -> Result<()> {
        poll.reregister(
            &self.socket,
            self.token,
            mio::Ready::readable(),
            mio::PollOpt::level(),
        )
    }

    #[inline]
    fn deregister(&mut self, poll: &mio::Poll) -> Result<()> {
        poll.deregister(&self.socket)
    }

    #[inline]
    fn reader(&mut self) -> &mut TcpStream {
        &mut self.socket
    }

    #[inline]
    fn read_token(&self) -> mio::Token {
        self.token
    }

    #[inline]
    fn writer(&mut self) -> &mut TcpStream {
        &mut self.socket
    }

    #[inline]
    fn write_token(&self) -> mio::Token {
        self.token
    }
}

impl EventedPty for Pty {
    #[inline]
    fn next_child_event(&mut self) -> Option<ChildEvent> {
        None
    }

    #[inline]
    fn child_event_token(&self) -> mio::Token {
        self.token
    }
}

impl OnResize for Pty {
    /// Resize the PTY.
    ///
    /// Tells the kernel that the window size changed with the new pixel
    /// dimensions and line/column counts.
    fn on_resize(&mut self, _window_size: WindowSize) {
    }
}