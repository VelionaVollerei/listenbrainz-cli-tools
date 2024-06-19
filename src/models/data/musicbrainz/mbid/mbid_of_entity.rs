use super::generic_mbid::EntityWithMBID;
use super::generic_mbid::MBIDSpe;
use super::generic_mbid::PrimaryID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait MBIDOfEntity<T, LID>
where
    Self: Clone + Into<LID>,
    T: EntityWithMBID + MBCached<LID>,
    LID: IsMbid<T> + Serialize + DeserializeOwned,
{
    fn into_legacy(self) -> LID {
        self.into()
    }

    async fn get_or_fetch_entity(&self) -> color_eyre::Result<T> {
        T::get_cached_or_fetch(&self.clone().into_legacy()).await
    }

    async fn into_primary(self) -> color_eyre::Result<MBIDSpe<T, PrimaryID>> {
        Self::from_legacy(&self.into_legacy()).await
    }

    async fn from_legacy(id: &LID) -> color_eyre::Result<MBIDSpe<T, PrimaryID>> {
        let primary_alias = T::get_cache().get_or_fetch_primary_mbid_alias(id).await?;

        Ok(MBIDSpe::from(primary_alias.to_string()))
    }
}
