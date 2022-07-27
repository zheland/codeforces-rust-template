use std::io::{Read, Result as IoResult, Write};
use std::mem::take;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::time::Duration;

pub struct ChannelWriter {
    send: Sender<Box<[u8]>>,
    buffer: Vec<u8>,
}

pub struct ChannelReader {
    recv: Receiver<Box<[u8]>>,
    buffer: Box<[u8]>,
    offset: usize,
}

impl ChannelWriter {
    pub fn new(send: Sender<Box<[u8]>>) -> Self {
        Self {
            send,
            buffer: Vec::new(),
        }
    }
}

impl ChannelReader {
    pub fn new(recv: Receiver<Box<[u8]>>) -> Self {
        Self {
            recv,
            buffer: Box::new([]),
            offset: 0,
        }
    }
}

impl Write for ChannelWriter {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.buffer.extend(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> IoResult<()> {
        if self.buffer.len() > 0 {
            let buffer = take(&mut self.buffer);
            self.send.send(buffer.into_boxed_slice()).unwrap();
        }
        Ok(())
    }
}

impl Drop for ChannelWriter {
    fn drop(&mut self) {
        if self.buffer.len() > 0 {
            let buffer = take(&mut self.buffer);
            let _ = self.send.send(buffer.into_boxed_slice()).ok();
        }
    }
}

impl Read for ChannelReader {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        loop {
            if self.offset >= self.buffer.len() {
                match self.recv.recv_timeout(Duration::from_secs(1)) {
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
