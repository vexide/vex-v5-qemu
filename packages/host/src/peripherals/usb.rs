use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

use tokio::{
    io::{AsyncRead, ReadBuf},
    sync::mpsc::Receiver,
};

/// Allows access to the simulated USB serial port.
#[derive(Debug)]
pub struct Usb {
    rx: Receiver<Vec<u8>>,
    read_buf: Vec<u8>,
}

impl Usb {
    pub const fn new(rx: Receiver<Vec<u8>>) -> Self {
        Self {
            rx,
            read_buf: Vec::new(),
        }
    }
}

impl AsyncRead for Usb {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let poll = self.rx.poll_recv(cx);
        let eof = match poll {
            Poll::Ready(Some(data)) => {
                self.read_buf.extend(data);
                false
            }
            Poll::Pending => false,
            Poll::Ready(None) => true,
        };

        let read_len = buf.remaining().min(self.read_buf.len());
        if read_len == 0 && !eof {
            return Poll::Pending;
        }

        buf.put_slice(&self.read_buf[..read_len]);
        self.read_buf.drain(0..read_len);

        Poll::Ready(Ok(()))
    }
}
