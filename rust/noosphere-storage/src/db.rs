use anyhow::{anyhow, Result};
use async_trait::async_trait;
use cid::Cid;
use libipld_core::{
    codec::{Codec, Decode, Encode, References},
    ipld::Ipld,
    raw::RawCodec,
};
use std::{collections::BTreeSet, fmt::Debug};
use tokio_stream::{Stream};
use ucan::store::{UcanStore, UcanStoreConditionalSend};

use crate::interface::{BlockStore, KeyValueStore, StorageProvider, Store};

use async_stream::try_stream;

#[cfg(not(target_arch = "wasm32"))]
pub trait SphereDbSendSync: Send + Sync {}

#[cfg(not(target_arch = "wasm32"))]
impl<T> SphereDbSendSync for T where T: Send + Sync {}

#[cfg(target_arch = "wasm32")]
pub trait SphereDbSendSync {}

#[cfg(target_arch = "wasm32")]
impl<T> SphereDbSendSync for T {}

pub const BLOCK_STORE: &str = "blocks";
pub const LINK_STORE: &str = "links";
pub const VERSION_STORE: &str = "versions";

#[derive(Clone)]
pub struct SphereDb<S>
where
    S: Store,
{
    block_store: S,
    link_store: S,
    version_store: S,
}

impl<S> SphereDb<S>
where
    S: Store,
{
    pub async fn new<P: StorageProvider<S>>(storage_provider: &P) -> Result<SphereDb<S>> {
        Ok(SphereDb {
            block_store: storage_provider.get_store(BLOCK_STORE).await?,
            link_store: storage_provider.get_store(LINK_STORE).await?,
            version_store: storage_provider.get_store(VERSION_STORE).await?,
        })
    }

    pub async fn set_version(&mut self, identity: &str, version: &Cid) -> Result<()> {
        self.version_store
            .set_key(identity.to_string(), version)
            .await
    }

    pub async fn get_version(&self, identity: &str) -> Result<Option<Cid>> {
        self.version_store.get_key(identity).await
    }

    pub async fn require_version(&self, identity: &str) -> Result<Cid> {
        self.version_store
            .get_key(identity)
            .await?
            .ok_or_else(|| anyhow!("No version was found for sphere {}", identity))
    }

    pub async fn get_block_links(&self, cid: &Cid) -> Result<Option<Vec<Cid>>> {
        self.link_store.get_key(&cid.to_string()).await
    }

    pub fn stream_links<'a>(&'a self, cid: &'a Cid) -> impl Stream<Item = Result<Cid>> + 'a {
        try_stream! {
            let mut visited_links = BTreeSet::new();
            let mut remaining_links = vec![cid.clone()];

            while let Some(cid) = remaining_links.pop() {
                if visited_links.contains(&cid) {
                    continue;
                }

                if let Some(mut links) = self.get_block_links(&cid).await? {
                    remaining_links.append(&mut links);
                }

                visited_links.insert(cid.clone());

                yield cid;
            }
        }
    }

    pub fn stream_blocks<'a>(
        &'a self,
        cid: &'a Cid,
    ) -> impl Stream<Item = Result<(Cid, Vec<u8>)>> + 'a {
        try_stream! {
            for await cid in self.stream_links(cid) {
                let cid = cid?;
                if let Some(block) = self.block_store.get_block(&cid).await? {
                    yield (cid, block);
                }
            }
        }
    }

    pub fn to_block_store(&self) -> S {
        self.block_store.clone()
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<S> BlockStore for SphereDb<S>
where
    S: Store,
{
    async fn put_links<C>(&mut self, cid: &Cid, block: &[u8]) -> Result<()>
    where
        C: Codec + Default,
        Ipld: References<C>,
    {
        let codec = C::default();
        let mut links = Vec::new();

        codec.references::<Ipld, _>(block, &mut links)?;

        self.link_store.set_key(&cid.to_string(), links).await?;

        Ok(())
    }

    async fn put_block(&mut self, cid: &cid::Cid, block: &[u8]) -> Result<()> {
        self.block_store.put_block(cid, block).await
    }

    async fn get_block(&self, cid: &cid::Cid) -> Result<Option<Vec<u8>>> {
        self.block_store.get_block(cid).await
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<S> UcanStore for SphereDb<S>
where
    S: Store,
{
    async fn read<T: Decode<RawCodec>>(&self, cid: &Cid) -> Result<Option<T>> {
        self.get::<RawCodec, T>(cid).await
    }

    async fn write<T: Encode<RawCodec> + UcanStoreConditionalSend + Debug>(
        &mut self,
        token: T,
    ) -> Result<Cid> {
        self.put::<RawCodec, T>(token).await
    }
}

#[cfg(test)]
mod tests {

    use libipld_cbor::DagCborCodec;
    use libipld_core::{ipld::Ipld, raw::RawCodec};
    use ucan::store::UcanJwtStore;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    use crate::{
        db::SphereDb,
        encoding::{block_encode, derive_cid},
        interface::BlockStore,
        memory::MemoryStorageProvider,
    };

    use tokio_stream::StreamExt;

    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    pub async fn it_stores_links_when_a_block_is_saved() {
        let storage_provider = MemoryStorageProvider::default();
        let mut db = SphereDb::new(&storage_provider).await.unwrap();

        let list1 = vec!["cats", "dogs", "pigeons"];
        let list2 = vec!["apples", "oranges", "starfruit"];

        let cid1 = db.save::<DagCborCodec, _>(&list1).await.unwrap();
        let cid2 = db.save::<DagCborCodec, _>(&list2).await.unwrap();

        let list3 = vec![cid1, cid2];

        let cid3 = db.save::<DagCborCodec, _>(&list3).await.unwrap();

        let links = db.get_block_links(&cid3).await.unwrap();

        assert_eq!(Some(list3), links);
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    pub async fn it_can_stream_all_blocks_in_a_dag() {
        let storage_provider = MemoryStorageProvider::default();
        let mut db = SphereDb::new(&storage_provider).await.unwrap();

        let list1 = vec!["cats", "dogs", "pigeons"];
        let list2 = vec!["apples", "oranges", "starfruit"];

        let cid1 = db.save::<DagCborCodec, _>(&list1).await.unwrap();
        let cid2 = db.save::<DagCborCodec, _>(&list2).await.unwrap();

        let list3 = vec![cid1, cid2];

        let cid3 = db.save::<DagCborCodec, _>(&list3).await.unwrap();

        let stream = db.stream_blocks(&cid3);

        tokio::pin!(stream);

        let mut cids = Vec::new();

        while let Some((cid, block)) = stream.try_next().await.unwrap() {
            let derived_cid = derive_cid::<DagCborCodec>(&block);
            assert_eq!(cid, derived_cid);
            cids.push(cid);
        }

        assert_eq!(cids.len(), 3);

        for cid in [cid1, cid2, cid3] {
            assert!(cids.contains(&cid));
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    pub async fn it_can_put_a_raw_block_and_read_it_as_a_token() {
        let storage_provider = MemoryStorageProvider::default();
        let mut db = SphereDb::new(&storage_provider).await.unwrap();

        let (cid, block) = block_encode::<RawCodec, _>(&Ipld::Bytes(b"foobar".to_vec())).unwrap();

        db.put_block(&cid, &block).await.unwrap();

        let token = db.read_token(&cid).await.unwrap();

        assert_eq!(token, Some("foobar".into()));
    }
}