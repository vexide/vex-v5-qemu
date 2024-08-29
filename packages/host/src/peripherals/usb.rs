use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

use tokio::{
    io::{AsyncRead, ReadBuf},
    sync::mpsc::{error::TryRecvError, Receiver},
};

#[derive(Debug)]
pub struct Usb {
    rx: Receiver<Vec<u8>>,
    read_buf: Vec<u8>,
}

impl Usb {
    pub fn new(rx: Receiver<Vec<u8>>) -> Self {
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
        log::debug!("Usb poll_read");
        match self.rx.poll_recv(cx) {
            Poll::Ready(Some(data)) => {
                log::debug!("poll_recv: New data");
                self.read_buf.extend(data);

                let read_len = buf.remaining().min(self.read_buf.len());
                buf.put_slice(&self.read_buf[..read_len]);
                self.read_buf.drain(0..read_len);

                Poll::Ready(Ok(()))
            }
            Poll::Pending => {
                let read_len = buf.remaining().min(self.read_buf.len());
                if read_len > 0 {
                    log::debug!("poll_recv: No new data, but have some buffered");
                    buf.put_slice(&self.read_buf[..read_len]);
                    self.read_buf.drain(0..read_len);
                    Poll::Ready(Ok(()))
                } else {
                    log::debug!("poll_recv: No data");
                    Poll::Pending
                }
            }
            Poll::Ready(None) => Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "Sender was dropped.",
            ))),
        }
    }
}
