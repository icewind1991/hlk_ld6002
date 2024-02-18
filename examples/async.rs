use embedded_io_adapters::tokio_1::FromTokio;
use hlk_ld6002::{AsyncMessageStream, Data};
use serialport::SerialPort;
use std::env::args;
use std::time::{Duration, Instant};
use tokio_serial::{ClearBuffer, SerialPortBuilderExt};

#[tokio::main]
async fn main() {
    let port = args().nth(1).expect("no port provided");
    let port = tokio_serial::new(&port, 1_382_400)
        .timeout(Duration::from_millis(50))
        .open_native_async()
        .expect("Failed to open port");
    port.clear(ClearBuffer::All).unwrap();

    let mut messages = AsyncMessageStream::new(FromTokio::new(port));

    let mut data = Data::default();

    let mut last = Instant::now();

    print!("{}", termion::cursor::Save);

    loop {
        let message = messages.next().await;
        if let Ok(message) = message {
            data.update(message);
            if last.elapsed() > Duration::from_millis(100) {
                last = Instant::now();
                print!("{:?}{}", data, termion::cursor::Restore);
            }
        }
    }
}
