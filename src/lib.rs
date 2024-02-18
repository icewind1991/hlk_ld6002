#![no_std]

use binrw::error::Error as RwError;
use binrw::io::{NoSeek, Read, Seek};
use binrw::prelude::*;
use binrw::{Endian, VecArgs};
use bytemuck::{cast, cast_slice};

#[derive(Debug)]
pub enum Error {
    InvalidDataLength {
        expected: u16,
        got: u16,
        ty: MessageType,
    },
}

#[derive(Debug, Clone, BinRead, Copy)]
#[repr(u16)]
#[brw(big, repr(u16))]
pub enum MessageType {
    Phase = 0x0a13,
    Respiratory = 0x0a14,
    Heartbeat = 0x0a15,
    Distance = 0x0a16,
}

impl MessageType {
    fn expected_length(self) -> u16 {
        match self {
            MessageType::Phase => 12,
            MessageType::Respiratory => 4,
            MessageType::Heartbeat => 4,
            MessageType::Distance => 8,
        }
    }
}

#[derive(Clone, Debug, BinRead)]
#[brw(big)]
#[allow(dead_code)]
struct Frame {
    id: u16,
    #[br(assert(length <= 16))]
    length: u16,
    ty: MessageType,
    header_checksum: u8,
    #[br(count = length)]
    data: FrameData<16>,
    data_checksum: u8,
}

#[derive(Debug, Clone)]
struct FrameData<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> FrameData<N> {
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<const N: usize> AsRef<[u8]> for FrameData<N> {
    fn as_ref(&self) -> &[u8] {
        &self.data[0..self.len]
    }
}

impl<const N: usize> BinRead for FrameData<N> {
    type Args<'a> = VecArgs<u8>;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        if args.count > N {
            return Err(RwError::AssertFail {
                pos: 0,
                message: "".into(),
            });
        }
        let mut data = [0u8; N];
        for byte in data.iter_mut().take(args.count) {
            *byte = reader.read_be()?;
        }
        Ok(FrameData {
            data,
            len: args.count,
        })
    }
}

impl Frame {
    fn body(&self) -> Result<MessageBody, Error> {
        let numbers = cast_slice::<_, u32>(self.data.as_ref());

        match (self.ty, self.data.len()) {
            (MessageType::Phase, 12) => {
                let numbers: [u32; 3] = numbers.try_into().unwrap();
                Ok(MessageBody::Phase(cast(numbers)))
            }
            (MessageType::Respiratory, 4) => {
                Ok(MessageBody::Respiratory(f32::from_bits(numbers[0])))
            }
            (MessageType::Heartbeat, 4) => Ok(MessageBody::Heartbeat(f32::from_bits(numbers[0]))),
            (MessageType::Distance, 8) => {
                let distance = if numbers[0] == 1 {
                    f32::from_bits(numbers[1])
                } else {
                    0.0
                };
                Ok(MessageBody::Distance(Some(distance)))
            }
            (MessageType::Distance, 4) => Ok(MessageBody::Distance(None)),
            _ => Err(Error::InvalidDataLength {
                got: self.data.len() as u16,
                expected: self.ty.expected_length(),
                ty: self.ty,
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub enum MessageBody {
    Phase([f32; 3]),
    Respiratory(f32),
    Heartbeat(f32),
    Distance(Option<f32>),
}

pub struct MessageStream<R> {
    reader: NoSeek<R>,
}

impl<R: Read> MessageStream<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: NoSeek::new(reader),
        }
    }

    fn read(&mut self) -> Result<Frame, binrw::Error> {
        let mut byte = [0];
        loop {
            self.reader.read(&mut byte).ok();
            if byte[0] == 0x01 {
                return Frame::read(&mut self.reader);
            }
        }
    }
}

impl<R: Read> Iterator for MessageStream<R> {
    type Item = MessageBody;

    fn next(&mut self) -> Option<Self::Item> {
        let frame = self.read().ok()?;

        frame.body().ok()
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Data {
    respiratory: f32,
    distance: f32,
    heartbeat: f32,
}

impl Data {
    pub fn update(&mut self, message: MessageBody) {
        match message {
            MessageBody::Respiratory(rate) => {
                self.respiratory = rate;
            }
            MessageBody::Distance(Some(distance)) if distance > 0.0 => {
                self.distance = distance;
            }
            MessageBody::Heartbeat(rate) => {
                self.heartbeat = rate;
            }
            _ => {}
        }
    }
}
