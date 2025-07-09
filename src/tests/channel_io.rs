use core::mem::take;
use core::time::Duration;
use std::io::{Read, Result as IoResult, Write};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};

#[derive(Debug)]
pub struct ChannelWriter {
    send: Sender<Box<[u8]>>,
    buffer: Vec<u8>,
}

#[derive(Debug)]
pub struct ChannelReader {
    recv: Receiver<Box<[u8]>>,
    buffer: Box<[u8]>,
    offset: usize,
    timeout: Duration,
}

impl ChannelWriter {
    #[must_use]
    pub fn new(send: Sender<Box<[u8]>>) -> Self {
        Self {
            send,
            buffer: Vec::new(),
        }
    }
}

impl ChannelReader {
    #[must_use]
    pub fn new(recv: Receiver<Box<[u8]>>, timeout: Duration) -> Self {
        Self {
            recv,
            buffer: Box::new([]),
            offset: 0,
            timeout,
        }
    }
}

impl Write for ChannelWriter {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.buffer.extend(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> IoResult<()> {
        if !self.buffer.is_empty() {
            let buffer = take(&mut self.buffer);
            self.send.send(buffer.into_boxed_slice()).unwrap();
        }
        Ok(())
    }
}

impl Drop for ChannelWriter {
    fn drop(&mut self) {
        if !self.buffer.is_empty() {
            let buffer = take(&mut self.buffer);
            let _ = self.send.send(buffer.into_boxed_slice()).ok();
        }
    }
}

impl Read for ChannelReader {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        loop {
            if self.offset >= self.buffer.len() {
                match self.recv.recv_timeout(self.timeout) {
                    Ok(buffer) => {
                        self.buffer = buffer;
                        self.offset = 0;
                    }
                    Err(RecvTimeoutError::Timeout) => panic!("channel reader timeout"),
                    Err(RecvTimeoutError::Disconnected) => break Ok(0),
                }
            } else {
                let len = (self.buffer.len() - self.offset).min(buf.len());
                buf[0..len].copy_from_slice(&self.buffer[self.offset..self.offset + len]);
                self.offset += len;
                break Ok(len);
            }
        }
    }
}
