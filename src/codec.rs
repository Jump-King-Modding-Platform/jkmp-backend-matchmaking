use bincode::{
    config::{
        Bounded, FixintEncoding, LittleEndian, WithOtherEndian, WithOtherIntEncoding,
        WithOtherLimit,
    },
    DefaultOptions, Options,
};
use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::messages::Message;

pub struct MessagesCodec {}

impl MessagesCodec {
    pub fn new() -> Self {
        Self {}
    }
}

fn get_options() -> WithOtherIntEncoding<
    WithOtherLimit<WithOtherEndian<DefaultOptions, LittleEndian>, Bounded>,
    FixintEncoding,
> {
    DefaultOptions::new()
        .with_little_endian()
        .with_limit(4096)
        .with_fixint_encoding()
}

impl Encoder<Message> for MessagesCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let payload = get_options().serialize(&item)?;

        dst.put_u32_le(payload.len() as u32);
        dst.put_slice(&payload);

        Ok(())
    }
}

impl Decoder for MessagesCodec {
    type Item = Message;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }

        if src.remaining() < 4 {
            src.advance(src.remaining());
            anyhow::bail!("Message is too small (<4 bytes)");
        }

        let length = src.get_u32_le() as usize;
        let remaining = src.remaining();

        if length as usize > remaining {
            anyhow::bail!(
                "Message length ({}) > Remaining ({})",
                length,
                src.remaining()
            );
        }

        if length == 0 {
            anyhow::bail!("Message length is zero");
        }

        let message: Self::Item = get_options().deserialize(&src[..length])?;
        src.advance(length);

        Ok(Some(message))
    }
}
