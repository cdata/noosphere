use async_stream::try_stream;
use bytes::Bytes;
use cid::Cid;
use noosphere::data::BodyChunkIpld;
use noosphere_storage::interface::{DagCborStore, Store};
use tokio_stream::Stream;

/// Helper to easily decode a linked list of `BodyChunkIpld` as a byte stream
pub struct BodyChunkDecoder<'a, 'b, S: Store>(pub &'a Cid, pub &'b S);

impl<'a, 'b, S: Store> BodyChunkDecoder<'a, 'b, S> {
    pub fn stream(self) -> impl Stream<Item = Result<Bytes, std::io::Error>> + Unpin {
        let mut next = Some(self.0.clone());
        let store = self.1.clone();
        Box::pin(try_stream! {
            while let Some(cid) = next {
                let chunk: BodyChunkIpld = store.load(&cid).await.map_err(|error| {
                    std::io::Error::new(std::io::ErrorKind::UnexpectedEof, error.to_string())
                })?;
                yield Bytes::from(chunk.bytes);
                next = chunk.next;
            }
        })
    }
}