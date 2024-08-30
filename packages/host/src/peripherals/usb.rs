use std::{
    io, pin::Pin, task::{Context, Poll}
};

use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    sync::mpsc::{error::{RecvError, SendError, TrySendError}, Receiver, Sender},
};
use vex_v5_qemu_protocol::KernelBoundPacket;

/// Allows access to the simulated USB serial port.
#[derive(Debug)]
pub struct Usb {
    tx: Sender<KernelBoundPacket>,
    rx: Receiver<Vec<u8>>,
    read_buf: Vec<u8>,
}

impl Usb {
    pub(crate) const fn new(tx: Sender<KernelBoundPacket>, rx: Receiver<Vec<u8>>) -> Self {
        Self {
            tx,
            rx,
            read_buf: Vec::new(),
        }
    }

    pub async fn recv(&mut self) -> Option<Vec<u8>> {
        self.rx.recv().await
    }

    pub async fn send(&mut self, data: Vec<u8>) -> Result<(), SendError<KernelBoundPacket>> {
        self.tx.send(KernelBoundPacket::UsbSerial(data)).await?;
        Ok(())
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

impl AsyncWrite for Usb {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        match self.tx.try_send(KernelBoundPacket::UsbSerial(buf.to_vec())) {
            Ok(()) => Poll::Ready(Ok(buf.len())),
            Err(TrySendError::Full(_)) => {
                // TODO: determine if this hogs CPU cycles
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(TrySendError::Closed(_)) => Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "Sender was dropped.",
            ))),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
}
