use std::io::{BufReader, BufWriter};
use std::sync::mpsc::channel;
use std::thread;

use crate::tests::{ChannelReader, ChannelWriter};
use crate::Io;

pub fn test_with_interactor<F1, F2>(problem: F1, interactor: F2)
where
    F1: FnOnce(&mut Io<BufReader<ChannelReader>, BufWriter<ChannelWriter>>) + Send + Sync,
    F2: FnOnce(&mut Io<BufReader<ChannelReader>, BufWriter<ChannelWriter>>) + Send + Sync,
{
    let (send, recv) = channel();
    let problem_reader = BufReader::new(ChannelReader::new(recv));
    let interactor_writer = BufWriter::new(ChannelWriter::new(send));
    let (send, recv) = channel();
    let interactor_reader = BufReader::new(ChannelReader::new(recv));
    let problem_writer = BufWriter::new(ChannelWriter::new(send));
    let mut problem_io = Io::new(problem_reader, problem_writer);
    let mut interactor_io = Io::new(interactor_reader, interactor_writer);
    thread::scope(|scope| {
        let _handle = scope.spawn(move || problem(&mut problem_io));
        let _handle = scope.spawn(move || interactor(&mut interactor_io));
    });
}
