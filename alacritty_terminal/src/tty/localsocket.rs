//! TTY related functionality.
use crate::config::{PtyConfig};
use crate::event::{OnResize, WindowSize};
use crate::tty::{ChildEvent, EventedPty, EventedReadWrite};
use mio::net::TcpStream;

use std::io::*;

pub struct Pty {
    socket : TcpStream,
    token: mio::Token,
    child_event_token : mio::Token
}

/// Create a new localsocket and return a handle to interact with it.
pub fn new(config: &PtyConfig, _window_size: WindowSize, _window_id: Option<usize>) -> Result<Pty> {

    let local_addr = format!("127.0.0.1:{}" , config.local_socket_port);

    let s = TcpStream::connect(&local_addr.parse().unwrap()).unwrap();
    Ok(Pty {
        socket : s,
        token: 0.into(),
        child_event_token : 0.into()
    })
}

impl EventedReadWrite for Pty {
    type Reader = TcpStream;
    type Writer = TcpStream;

    #[inline]
    fn register(
        &mut self,
        poll: &mio::Poll,
        token: &mut dyn Iterator<Item = mio::Token>,
        interest: mio::Ready,
        poll_opts: mio::PollOpt,
    ) -> Result<()> {
        self.token = token.next().unwrap();

        poll.register(&self.socket, self.token, interest, poll_opts)?;

        self.child_event_token = token.next().unwrap();
        Ok(())
    }

    #[inline]
    fn reregister(
        &mut self,
        poll: &mio::Poll,
        interest: mio::Ready,
        poll_opts: mio::PollOpt,
    ) -> Result<()> {
        poll.reregister(&self.socket, self.token, interest, poll_opts)?;
        Ok(())
    }

    #[inline]
    fn deregister(&mut self, poll: &mio::Poll) -> Result<()> {
        poll.deregister(&self.socket)?;
        Ok(())
    }

    #[inline]
    fn reader(&mut self) -> &mut Self::Reader {
        &mut self.socket
    }

    #[inline]
    fn read_token(&self) -> mio::Token {
        self.token
    }

    #[inline]
    fn writer(&mut self) -> &mut Self::Writer {
        &mut self.socket
    }

    #[inline]
    fn write_token(&self) -> mio::Token {
        self.token
    }
}

impl EventedPty for Pty {
    fn child_event_token(&self) -> mio::Token {
        self.child_event_token
    }

    fn next_child_event(&mut self) -> Option<ChildEvent> {
        None
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