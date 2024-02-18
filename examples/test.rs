use embedded_io::{ErrorType, Read, ReadExactError};
use hlk_ld6002::{Data, MessageStream};
use serialport::ClearBuffer;
use std::env::args;
use std::time::{Duration, Instant};

fn main() {
    let port = args().nth(1).expect("no port provided");
    let port = serialport::new(&port, 1_382_400)
        .timeout(Duration::from_millis(50))
        .open()
        .expect("Failed to open port");
    port.clear(ClearBuffer::All).expect("clear");

    let mut messages = MessageStream::new(ReadAdapter(port));

    let mut data = Data::default();

    let mut last = Instant::now();

    print!("{}", termion::cursor::Save);

    loop {
        if let Some(message) = messages.next() {
            if let Ok(message) = message {
                data.update(message);
                if last.elapsed() > Duration::from_millis(100) {
                    last = Instant::now();
                    print!("{:?}{}", data, termion::cursor::Restore);
                }
            }
        }
    }
}

struct ReadAdapter<R>(R);

impl<R: std::io::Read> ErrorType for ReadAdapter<R> {
    type Error = std::io::Error;
}

impl<R: std::io::Read> Read for ReadAdapter<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.0.read(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), ReadExactError<Self::Error>> {
        self.0.read_exact(buf).map_err(ReadExactError::Other)
    }
}
