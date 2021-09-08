use bincode::{
    config::{
        Bounded, LittleEndian, VarintEncoding, WithOtherEndian, WithOtherIntEncoding,
        WithOtherLimit,
    },
    DefaultOptions, Options,
};
use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::messages::Message;

pub struct MessagesCodec {
    options: WithOtherIntEncoding<
        WithOtherLimit<WithOtherEndian<DefaultOptions, LittleEndian>, Bounded>,
        VarintEncoding,
    >,
}

impl MessagesCodec {
    pub fn new() -> Self {
        Self {
            options: DefaultOptions::new()
                .with_little_endian()
                .with_limit(4096)
                .with_varint_encoding(),
        }
    }
}

impl Encoder<Message> for MessagesCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let payload = self.options.serialize(&item)?;

        crate::encoding::put_varint_le(dst, payload.len() as u64);
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

        let length = crate::encoding::get_varint_le(src)? as usize;
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

        let message: Self::Item = self.options.deserialize(&src[..length])?;
        src.advance(length);

        Ok(Some(message))
    }
}
