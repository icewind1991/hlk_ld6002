use std::env::args;
use std::time::{Duration, Instant};
use serialport::ClearBuffer;
use hlk_ld6002::{Data, MessageStream};


fn main() {
    let port = args().nth(1).expect("no port provided");
    let port = serialport::new(&port, 1_382_400)
        .timeout(Duration::from_millis(50))
        .open().expect("Failed to open port");
    port.clear(ClearBuffer::All).expect("clear");

    let mut messages = MessageStream::new(port);

    let mut data = Data::default();

    let mut last = Instant::now();

    print!("{}", termion::cursor::Save);

    loop {
        if let Some(message) = messages.next() {
            data.update(message);
            if last.elapsed() > Duration::from_millis(100) {
                last = Instant::now();
                print!("{:?}{}", data, termion::cursor::Restore);
            }
        }
    }
}