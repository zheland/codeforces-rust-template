use core::marker::PhantomData;
use core::mem::take;
use core::time::Duration;
use std::io::{BufReader, BufWriter, Read, Result as IoResult, Write};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};

mod template;

pub struct Problem<SolverFn, InteractorFn, PresetsBuilderFn, Preset> {
    name: &'static str,
    solver: SolverFn,
    interactor: InteractorFn,
    presets_buidler: PresetsBuilderFn,
    example_timeout: Duration,
    examples: &'static str,
    test_timeout: Duration,
    tests: &'static str,
    is_interactor_enabled: bool,
    interactive_timeout: Duration,
    preset_ty: PhantomData<Preset>,
}

pub fn check_problem<SolverFn, InteractorFn, PresetsBuilderFn, Preset>(
    problem: Problem<SolverFn, InteractorFn, PresetsBuilderFn, Preset>,
) {
}

macro_rules! build_problem {
    ( $name:ident ) => {
        Problem {
            name: stringify!($name),
            solver: $name::solver::<BufReader<ChannelReader>, BufWriter<ChannelWriter>>,
            interactor: $name::interactor::<BufReader<ChannelReader>, BufWriter<ChannelWriter>>,
            presets_buidler: $name::presets_buidler,
            example_timeout: $name::EXAMPLE_TIMEOUT,
            examples: $name::EXAMPLES,
            test_timeout: $name::TESTS_TIMEOUT,
            tests: $name::TESTS,
            is_interactor_enabled: $name::IS_INTERACTOR_ENABLED,
            interactive_timeout: $name::INTERACTIVE_TIMEOUT,
            preset_ty: PhantomData::<$name::Preset>,
        }
    };
}

pub fn main() {
    check_problem(build_problem!(template));
}

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
    pub const fn new(send: Sender<Box<[u8]>>) -> Self {
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
