use embedded_io_adapters::std::FromStd;
use hlk_ld6002::{Data, MessageStream};
use serialport::ClearBuffer;
use std::env::args;
use std::time::{Duration, Instant};

fn main() {
    let port = args().nth(1).expect("no port provided");
    let port = serialport::new(port, 1_382_400)
        .timeout(Duration::from_millis(50))
        .open()
        .expect("Failed to open port");
    port.clear(ClearBuffer::All).expect("clear");

    let messages = MessageStream::new(FromStd::new(port));

    let mut data = Data::default();

    let mut last = Instant::now();

    print!("{}", termion::cursor::Save);

    for message in messages.flatten() {
        data.update(message);
        if last.elapsed() > Duration::from_millis(100) {
            last = Instant::now();
            print!("{:?}{}", data, termion::cursor::Restore);
        }
    }
}
